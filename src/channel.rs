use libm::powf;

use crate::*;
use core::f32::consts::TAU;

const FREQ_C4: f32 = 261.63;

/// A single sound channel with configurable properties.
pub struct Channel {
    // Wavetable
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    wavetable: Vec<f32>,
    wave_out: f32,
    // Timing
    phase: f32,
    time: f32,
    time_env: f32,
    time_tone: f32,
    time_noise: f32,
    period: f32,
    // Volume
    /// Optional volume envelope, range is 0.0 ..= 1.0
    pub volume_env: Option<Envelope>,
    volume: f32,
    volume_attn: f32,
    // Pitch
    /// Optional pitch envelope. range is -1.0 ..= 1.0, multiplied by pitch_env_multiplier.
    /// Resulting value is added to the current note in MIDI note range (C4 = 60.0).
    pub pitch_env: Option<Envelope>,
    /// Multiplies the pitch envelope to obtain a pitch offset. Every time it doubles, the pitch envelope can reach
    /// a new octave, so 2.0 is one octave, 4.0 is two octaves, etc. Default is 1.0.
    pub pitch_env_multiplier: f32,
    // Volume tremolo
    /// Optional volume tremolo. Acts as a secondary envelope, added to the regular volume envelope.
    pub tremolo: Option<SpecsTremolo>,
    // Noise
    rng: Rng,
    noise_on: bool,
    noise_period: f32,
    noise_output: f32,
    // State
    specs: SpecsChip,
    pan: f32,
    midi_note: f32,
    playing: bool,
    left_mult: f32,
    right_mult: f32,
    last_sample_index: usize,
    last_sample_value: f32,
    // Envelope processing
    /// With the default value of Some(60.0) in Hertz, envelopes aren't processed on every sample.
    /// The last envelope value will be re-used during each period, which is very accurate to how
    /// many 80's and 90's games processed sound (i.e. once every video frame).
    /// Affects Volume and Pitch envelopes, as well as tremolo and vibratto.
    pub envelope_rate: Option<f32>,
    env_period: f32,
    last_env: LatestEnvelopes,
    last_env_time: f32,
}

impl From<SpecsChip> for Channel {
    fn from(specs: SpecsChip) -> Self {
        let mut result = Self {
            // Timing
            phase: 0.0,
            time: 0.0,
            time_env: 0.0,
            time_tone: 0.0,
            time_noise: 0.0,
            period: 1.0 / FREQ_C4,
            // Wavetable
            wavetable: Self::get_wavetable(&specs),
            wave_out: 0.0,
            // Volume
            volume: 1.0,
            volume_env: None,
            volume_attn: 0.0,
            // Pitch
            pitch_env: None,
            pitch_env_multiplier: 1.0,
            // Tremolo
            tremolo: None,
            // Noise
            rng: Self::get_rng(&specs),
            noise_on: false,
            noise_period: 0.0,
            noise_output: 0.0,
            // State
            specs,
            pan: 0.0,
            midi_note: 60.0,
            playing: false,
            left_mult: 0.5,
            right_mult: 0.5,
            last_sample_index: 0,
            last_sample_value: 0.0,
            // Envelope processing
            envelope_rate: Some(60.0),
            env_period: 0.0,
            last_env_time: 0.0,
            last_env: LatestEnvelopes::default(),
        };
        result.set_note(4, Note::C);
        result.set_volume(1.0);
        result.reset_time();
        result
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::from(SpecsChip::default())
    }
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new_psg(allow_noise: bool) -> Self {
        let specs = SpecsChip {
            wavetable: SpecsWavetable::psg(),
            pan: SpecsPan::psg(),
            pitch: SpecsPitch::psg(),
            volume: SpecsVolume::psg(),
            noise: SpecsNoise::psg(allow_noise),
        };
        Self::from(specs)
    }

    /// Creates a new channel configured with a 32 byte wavetable
    pub fn new_scc() -> Self {
        let specs = SpecsChip {
            wavetable: SpecsWavetable::scc(),
            pan: SpecsPan::scc(),
            pitch: SpecsPitch::scc(),
            volume: SpecsVolume::scc(),
            noise: SpecsNoise::None,
        };
        Self::from(specs)
    }

    /// Allows sound generation on this channel.
    pub fn play(&mut self) {
        self.playing = true;
        self.calculate_multipliers();
    }

    /// Stops sound generation on this channel.
    pub fn stop(&mut self) {
        self.playing = false;
        self.time = 0.0;
        self.time_noise = 0.0;
        self.time_env = 0.0;
    }

    /// Current playing state.
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// The virtual Chip specs used in this channel.
    pub fn specs(&self) -> &SpecsChip {
        &self.specs
    }

    /// True if channel is set to noise, false if set to tone.
    pub fn is_noise(&self) -> bool {
        self.noise_on
    }

    /// Current octave.
    pub fn octave(&self) -> i32 {
        libm::floorf(self.midi_note / 12.0) as i32 - 1
    }

    /// Current midi note (C4 = 60). Does not account for pitch envelope.
    pub fn note(&self) -> i32 {
        libm::floorf(self.midi_note) as i32 % 12
    }

    /// Current frequency. Does not account for pitch envelope.
    pub fn pitch(&self) -> f32 {
        1.0 / self.period
    }

    /// The main volume level. Values above 1.0 may cause clipping. Does not account for volume envelope.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Current stereo panning. Zero means centered (mono).
    pub fn pan(&self) -> f32 {
        self.pan
    }

    /// Mutable access to the wavetable. Be careful to not set values beyond -1.0 to 1.0.
    pub fn wavetable(&mut self) -> &mut Vec<f32> {
        &mut self.wavetable
    }

    /// Resets al internal timers (tone, noise, envelopes)
    pub fn reset_time(&mut self) {
        self.time = 0.0;
        self.time_tone = 0.0;
        self.time_noise = 0.0;
        self.reset_envelopes();
    }

    /// Resets just the envelope timer. Will cause the envelopes' state to revert to "Attack".
    pub fn reset_envelopes(&mut self) {
        self.time_env = 0.0;
        self.last_env_time = 0.0;
        if let Some(env) = &mut self.volume_env {
            env.reset();
        }
        if let Some(env) = &mut self.pitch_env {
            env.reset();
        }
    }

    /// TODO: Better noise Rng per noise settings
    pub fn set_specs(&mut self, specs: SpecsChip) {
        self.rng = Self::get_rng(&specs);
        self.specs = specs;
    }

    /// Resets the wavetable and copies new f32 values to it, ensuring -1.0 to 1.0 range.
    /// Will return an error if values are invalid.
    pub fn set_wavetable(&mut self, table: &[f32]) -> Result<(), ChipError> {
        self.wavetable.clear();
        for item in table {
            if *item >= -1.0 && *item <= 1.0 {
                self.wavetable.push(*item);
            } else {
                return Err(ChipError::InvalidWavetable);
            }
        }
        Ok(())
    }

    /// A value between 0.0 and 1.0. It will be quantized, receive a fixed gain and
    /// mapped to an exponential curve, according to the SpecsChip.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 16.0);
        self.calculate_multipliers();
    }

    /// Stereo panning, from left (-1.0) to right (1.0). Centered is zero. Will be quantized per SpecsChip.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
        self.calculate_multipliers();
    }

    /// Switches channel between tone and noise generation, if specs allow noise.
    pub fn set_noise(&mut self, state: bool) {
        if self.specs.noise != SpecsNoise::None {
            self.noise_on = state;
        }
    }

    /// Adjusts internal pitch values to correspond to octave and note ( where C = 0, C# = 1, etc.).
    /// "reset_time" forces the waveform to start from position 0, ignoring previous phase.
    pub fn set_note(&mut self, octave: impl Into<i32>, note: impl Into<i32>) {
        let midi_note = get_midi_note(octave, note);
        self.set_midi_note(midi_note as f32);
    }

    /// Same as set_note, but the notes are an f32 value which allows "in-between" notes, or pitch sliding,
    /// and uses MIDI codes instead of octave and note, i.e. C4 is MIDI code 60.
    pub fn set_midi_note(&mut self, note: impl Into<f32>) {
        let note: f32 = note.into();
        self.midi_note = note;
        // Calculate note frequency
        let frequency = note_to_frequency_f64(note);
        self.period = 1.0 / frequency;
        // Auto reset time if needed
        if !self.specs.wavetable.use_loop || !self.playing {
            // If looping isn't required, ensure sample will be played from beginning.
            // Also, if channel is not playing it means we'll start playing a cycle
            // from 0.0 to avoid clicks.
            self.time = 0.0;
            self.time_tone = 0.0;
        } else {
            // Adjust time to ensure continuous change (instead of abrupt change)
            self.time = self.phase * self.period;
            self.time_tone = self.phase * self.period;
        };
        // SpecsNoise
        match &self.specs.noise {
            SpecsNoise::Random { pitch, .. } | SpecsNoise::Melodic { pitch, .. } => {
                if let Some(steps) = &pitch.steps {
                    let freq_range = if let Some(range) = &pitch.range {
                        *range.start()..=*range.end()
                    } else {
                        // C0 to C10 in "scientific pitch"", roughly the human hearing range
                        16.0..=16384.0
                    };
                    let tone_freq = 1.0 / self.period;
                    let noise_freq = quantize_range(tone_freq, *steps, freq_range.clone());
                    let noise_period = 1.0 / noise_freq;
                    self.noise_period = noise_period / pitch.multiplier;
                } else {
                    self.noise_period = self.period / pitch.multiplier;
                }
            }
            _ => {}
        }
    }

    fn process_envelopes(&mut self) -> LatestEnvelopes {
        // Adjust volume with envelope
        let volume_env = if let Some(env) = &mut self.volume_env {
            env.process(self.time_env)
        } else {
            1.0
        };

        // Quantize volume (if needed) and apply log curve.
        let volume = libm::powf(
            if let Some(steps) = self.specs.volume.steps {
                quantize_range(self.volume * volume_env, steps, 0.0..=1.0)
            } else {
                self.volume * volume_env
            },
            self.specs.volume.exponent,
        );

        // Adjust periods with pitch envelope
        let (tone_period, noise_period) = if let Some(env) = &mut self.pitch_env {
            let value = env.process(self.time_env);

            let tone_period = self.period * powf(self.pitch_env_multiplier, -value);
            let noise_period = self.noise_period * powf(self.pitch_env_multiplier, -value);

            self.time_tone = self.phase * tone_period;

            (tone_period, noise_period)
        } else {
            (self.period, self.noise_period)
        };

        self.last_env_time = self.time;
        LatestEnvelopes {
            tone_period,
            noise_period,
            volume,
        }
    }

    #[inline(always)]
    /// Returns the current sample and advances the internal timer.
    pub(crate) fn sample(&mut self, delta_time: f32) -> Sample<f32> {
        // Always apply attenuation, so that values always drift to zero
        self.wave_out *= self.volume_attn;

        // Early return if not playing
        if !self.playing {
            return Sample {
                left: self.wave_out * self.left_mult,
                right: self.wave_out * self.right_mult,
            };
        }

        // Envelope processing
        if self.envelope_rate.is_some() {
            // If there's an envelope rate, calculate envelopes when needed
            if self.time - self.last_env_time >= self.env_period || self.time == 0.0 {
                self.last_env = self.process_envelopes();
            }
        } else {
            // If no envelope rate, always calculate envelopes
            self.last_env = self.process_envelopes();
        }

        // Generate noise level, will be mixed later
        self.noise_output = match &self.specs.noise {
            SpecsNoise::None => 0.0,
            SpecsNoise::Random { volume_steps, .. } | SpecsNoise::Melodic { volume_steps, .. } => {
                if self.time_noise >= self.last_env.noise_period {
                    self.time_noise = 0.0;
                    (quantize_range(self.rng.next_f32(), *volume_steps, 0.0..=1.0) * 2.0) - 1.0
                } else {
                    self.noise_output
                }
            }
            SpecsNoise::WaveTable { .. } => 0.0,
        };

        // Determine wavetable index
        let len = self.wavetable.len();
        let index = if self.specs.wavetable.use_loop {
            (self.phase * len as f32) as usize
        } else {
            ((self.phase * len as f32) as usize).clamp(0, len - 1) // TODO: Needs testing
        };

        // Obtain wavetable sample and set it to output
        if index != self.last_sample_index {
            self.last_sample_index = index;
            let wave = self.wavetable[index];
            let value = if let Some(steps) = self.specs.wavetable.steps {
                quantize_range(wave, steps, -1.0..=1.0)
            } else {
                wave
            };
            // Avoids resetting attenuation if value hasn't changed
            if value != self.last_sample_value {
                self.wave_out = value;
                self.last_sample_value = value;
            }
        }

        // Advance timers
        self.time += delta_time;
        self.time_noise += delta_time;
        self.time_env += delta_time;
        self.time_tone += delta_time;
        self.phase = if self.specs.wavetable.use_loop {
            (self.time_tone % self.last_env.tone_period) / self.last_env.tone_period
        } else {
            self.time_tone / self.last_env.tone_period // TODO: Needs testing
        };

        // Mix with noise (currently just overwrites). TODO: optional mix
        if self.noise_on {
            self.wave_out = self.noise_output;
        }

        // Apply volume and optionally clamp to positive values
        let output = if self.specs.volume.prevent_negative_values {
            self.wave_out.clamp(0.0, 1.0) * self.last_env.volume
        } else {
            self.wave_out * self.last_env.volume
        };

        // Return sample with volume and pan applied
        Sample {
            left: output * self.left_mult,
            right: output * self.right_mult,
        }
    }

    // Must be called after setting volume or pan.
    // Used to pre-calculate as many values as possible instead of doing it per sample, since
    // this function is called much less frequently (by orders of magnitude)
    pub(crate) fn calculate_multipliers(&mut self) {
        // Pre calculate this so we don't do it on every sample
        self.volume_attn = 1.0 - self.specs.volume.attenuation.clamp(0.0, 1.0);
        // Pan quantization
        let pan = if let Some(pan_steps) = self.specs.pan.steps {
            quantize_range(self.pan, pan_steps, -1.0..=1.0)
        } else {
            self.pan
        };
        // Is applying gain to the pan OK? Needs testing
        self.left_mult = ((pan - 1.0) / -2.0) * self.specs.volume.gain;
        self.right_mult = ((pan + 1.0) / 2.0) * self.specs.volume.gain;
        // Envelope period
        if let Some(env_freq) = self.envelope_rate {
            self.env_period = 1.0 / env_freq;
        }
    }

    // New Rng from specs
    fn get_rng(specs: &SpecsChip) -> Rng {
        match specs.noise {
            SpecsNoise::None | SpecsNoise::Random { .. } | SpecsNoise::WaveTable { .. } => {
                Rng::new(16, 1)
            }
            SpecsNoise::Melodic { lfsr_length, .. } => Rng::new(lfsr_length as u32, 1),
        }
    }

    // New Wavetable Vec from specs
    fn get_wavetable(specs: &SpecsChip) -> Vec<f32> {
        (0..specs.wavetable.sample_count)
            .map(|i| {
                // Default sine wave
                let a = (i as f32 / specs.wavetable.sample_count as f32) * TAU;
                libm::sinf(a)
            })
            .collect()
    }
}

#[inline(always)]
pub(crate) fn note_to_frequency_f64(note: f32) -> f32 {
    libm::powf(2.0, (note - 69.0) / 12.0) * 440.0
}

#[inline(always)]
pub(crate) fn note_to_frequency_f32(note: f32) -> f32 {
    libm::powf(2.0, (note - 69.0) / 12.0) * 440.0
}

#[derive(Default)]
struct LatestEnvelopes {
    volume: f32,
    tone_period: f32,
    noise_period: f32,
}
