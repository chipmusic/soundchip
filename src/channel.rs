use libm::pow;

use crate::*;
use core::f32::consts::TAU;

const FREQ_C4: f64 = 261.63;

/// A single sound channel with configurable properties.
pub struct Channel {
    // Wavetable
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    pub wave_loop: bool,
    wavetable: Vec<f32>,
    wave_out: f32,
    // Timing. All timing values are f64, sample values are f32
    time: f64,
    time_env: f32,
    time_noise: f64,
    period: f64,
    // Wavetable
    // Volume
    /// Optional volume envelope, range is 0.0 ..= 1.0
    pub volume_env: Option<Envelope>,
    volume: f32,
    volume_attn: f32,
    // Pitch
    /// Optional pitch envelope. range is -1.0 ..= 1.0, multiplied by pitch_env_multiplier.
    /// Resulting value is added to the current note in MIDI note range (C4 = 60.0).
    pub pitch_env: Option<Envelope>,
    /// Multiplies the pitch envelope to obtain a pitch offset.
    pub pitch_env_multiplier:f32,
    // Noise
    rng: Rng,
    noise_on: bool,
    noise_period: f64,
    noise_output: f32,
    // State
    specs: ChipSpecs,
    pan: f32,
    midi_note: f32,
    playing: bool,
    left_mult: f32,
    right_mult: f32,
    last_sample_index: usize,
    last_sample_value: f32,

}

impl From<ChipSpecs> for Channel {
    fn from(specs: ChipSpecs) -> Self {
        let mut result = Self {
            // Timing
            time: 0.0,
            time_env: 0.0,
            time_noise: 0.0,
            period: 1.0 / FREQ_C4,
            // Wavetable
            wavetable: Self::get_wavetable(&specs),
            wave_loop: true,
            wave_out: 0.0,
            // Volume
            volume: 1.0,
            volume_env: None,
            volume_attn: 0.0,
            // Pitch
            pitch_env: None,
            pitch_env_multiplier: 1.0,
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
        };
        result.set_note(4, Note::C, true);
        result.set_volume(1.0);
        result
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::from(ChipSpecs::default())
    }
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new_psg(allow_noise: bool) -> Self {
        let specs = ChipSpecs {
            wavetable: WavetableSpecs::psg(),
            pan: PanSpecs::psg(),
            pitch: PitchSpecs::psg(),
            volume: VolumeSpecs::psg(),
            noise: NoiseSpecs::psg(allow_noise),
        };
        Self::from(specs)
    }

    /// Creates a new channel configured with a 32 byte wavetable
    pub fn new_scc() -> Self {
        let specs = ChipSpecs {
            wavetable: WavetableSpecs::scc(),
            pan: PanSpecs::scc(),
            pitch: PitchSpecs::scc(),
            volume: VolumeSpecs::scc(),
            noise: NoiseSpecs::None,
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
    pub fn specs(&self) -> &ChipSpecs {
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

    /// Current midi note (C4 = 60).
    pub fn note(&self) -> i32 {
        libm::floorf(self.midi_note) as i32 % 12
    }

    /// Current frequency.
    pub fn pitch(&self) -> f32 {
        1.0 / self.period as f32
    }

    /// The main volume level. Values above 1.0 may cause clipping.
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

    pub fn reset_envelopes(&mut self) {
        self.time_env = 0.0;
        if let Some(env) = &mut self.volume_env {
            env.reset();
        }
        if let Some(env) = &mut self.pitch_env {
            env.reset();
        }
    }

    /// TODO: Better noise Rng per noise settings
    pub fn set_specs(&mut self, specs: ChipSpecs) {
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
    /// mapped to an exponential curve, according to the ChipSpecs.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 16.0);
        self.calculate_multipliers();
    }

    /// Stereo panning, from left (-1.0) to right (1.0). Centered is zero. Will be quantized per ChipSpecs.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
        self.calculate_multipliers();
    }

    /// Switches channel between tone and noise generation, if specs allow noise.
    pub fn set_noise(&mut self, state: bool) {
        if self.specs.noise != NoiseSpecs::None {
            self.noise_on = state;
        }
    }

    /// Adjusts internal pitch values to correspond to octave and note ( where C = 0, C# = 1, etc.).
    /// "reset_time" forces the waveform to start from position 0, ignoring previous phase.
    pub fn set_note(&mut self, octave: impl Into<i32>, note: impl Into<i32>, reset_time: bool) {
        let midi_note = get_midi_note(octave, note) as f64;
        self.set_midi_note(midi_note, reset_time);
    }

    /// Same as set_note, but the notes are an f32 value which allows "in-between" notes, or pitch sliding,
    /// and uses MIDI codes instead of octave and note, i.e. C4 is MIDI code 60.
    pub fn set_midi_note(&mut self, note: impl Into<f64>, reset_time: bool) {
        let note: f64 = note.into();
        self.midi_note = note as f32;
        // cache current phase to re-apply at the end
        let previous_phase = (self.time % self.period as f64) / self.period as f64;
        // Calculate note frequency
        let frequency = note_to_frequency_f64(note);
        self.period = 1.0 / frequency;
        // Envelope timer
        if reset_time {
            self.time_env = 0.0;
        }
        self.time = if !self.wave_loop || !self.playing || reset_time {
            // If looping isn't required, ensure sample will be played from beginning.
            // Also, if channel is not playing it means we'll start playing a cycle
            // from 0.0 to avoid clicks.
            0.0
        } else {
            // Adjust time to ensure continuous change (instead of abrupt change)
            previous_phase * self.period
        };
        // NoiseSpecs
        match &self.specs.noise {
            NoiseSpecs::Random { pitch, .. } | NoiseSpecs::Melodic { pitch, .. } => {
                if let Some(steps) = &pitch.steps {
                    let freq_range = if let Some(range) = &pitch.range {
                        *range.start() as f64..=*range.end() as f64
                    } else {
                        // C0 to C10 in "scientific pitch"", roughly the human hearing range
                        16.0..=16384.0
                    };
                    let tone_freq = 1.0 / self.period;
                    let noise_freq = quantize_range_f64(tone_freq, *steps, freq_range.clone());
                    let noise_period = 1.0 / noise_freq;
                    self.noise_period = noise_period / pitch.multiplier as f64;
                } else {
                    self.noise_period = self.period / pitch.multiplier as f64;
                }
            }
            _ => {}
        }
    }

    #[inline(always)]
    /// Returns the current sample and advances the internal timer.
    pub(crate) fn sample(&mut self, delta_time: f64) -> Sample<f32> {
        // Always apply attenuation, so that values always drift to zero
        self.wave_out *= self.volume_attn;

        // Early return if not playing
        if !self.playing {
            return Sample {
                left: self.wave_out * self.left_mult,
                right: self.wave_out * self.right_mult,
            };
        }

        // Adjust volume with envelope
        let volume_env = if let Some(env) = &mut self.volume_env {
            env.process(self.time_env)
        } else {
            1.0
        };

        // Adjust periods with pitch envelope
        let (period, noise_period) = if let Some(env) = &mut self.pitch_env {
            let value = env.process(self.time_env) as f64;
            // let octave_mult = envelope_value * self.pitch_env_multiplier as f64;
            let tone_period = self.period * pow(self.pitch_env_multiplier as f64, -value);
            let noise_period = self.noise_period * pow(self.pitch_env_multiplier as f64, -value);
            // let note = self.note + offset;
            (tone_period, noise_period)
        } else {
            (self.period, self.noise_period)
        };

        // Generate noise level, will be mixed later
        self.noise_output = match &self.specs.noise {
            NoiseSpecs::None => 0.0,
            NoiseSpecs::Random { volume_steps, .. } | NoiseSpecs::Melodic { volume_steps, .. } => {
                if self.time_noise >= noise_period {
                    self.time_noise = 0.0;
                    (quantize_range_f32(self.rng.next_f32(), *volume_steps, 0.0..=1.0) * 2.0) - 1.0
                } else {
                    self.noise_output
                }
            }
            NoiseSpecs::WaveTable { .. } => 0.0,
        };

        // Determine wavetable index
        let len = self.wavetable.len();
        let index = if self.wave_loop {
            let phase = (self.time % period) / period;
            (phase * len as f64) as usize
        } else {
            let phase = (self.time / period).clamp(0.0, 1.0);
            ((phase * len as f64) as usize).clamp(0, len - 1)
        };

        // Obtain wavetable sample and set it to output
        if index != self.last_sample_index {
            self.last_sample_index = index;
            // TODO: Optional quantization!
            let value = if let Some(steps) = self.specs.wavetable.steps {
                quantize_range_f32(self.wavetable[index] as f32, steps, -1.0..=1.0)
            } else {
                self.wavetable[index] as f32
            };

            // Avoids resetting attenuation if value hasn't changed
            if value != self.last_sample_value {
                self.wave_out = value;
                self.last_sample_value = value;
            }
        }

        // Advance timer
        self.time += delta_time;
        self.time_noise += delta_time;
        self.time_env += delta_time as f32;

        // Mix with noise (currently just overwrites). TODO: optional mix
        if self.noise_on {
            self.wave_out = self.noise_output;
        }

        // Quantize volume (if needed) and apply log curve.
        // Any math using the envelope needs to be calculate per sample, unfortunately.
        let vol = libm::powf(
            if let Some(steps) = self.specs.volume.steps {
                quantize_range_f32(self.volume * volume_env, steps, 0.0..=1.0)
            } else {
                self.volume * volume_env
            },
            self.specs.volume.exponent,
        );

        // Apply volume and optionally clamp to positive values
        let output = if self.specs.volume.prevent_negative_values {
            self.wave_out.clamp(0.0, 1.0) * vol
        } else {
            self.wave_out * vol
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
            quantize_range_f32(self.pan, pan_steps, -1.0..=1.0)
        } else {
            self.pan
        };
        // Is applying gain to the pan OK? Needs testing
        self.left_mult = ((pan - 1.0) / -2.0) * self.specs.volume.gain;
        self.right_mult = ((pan + 1.0) / 2.0) * self.specs.volume.gain;
    }

    // New Rng from specs
    fn get_rng(specs: &ChipSpecs) -> Rng {
        match specs.noise {
            NoiseSpecs::None | NoiseSpecs::Random { .. } | NoiseSpecs::WaveTable { .. } => {
                Rng::new(16, 1)
            }
            NoiseSpecs::Melodic { lfsr_length, .. } => Rng::new(lfsr_length as u32, 1),
        }
    }

    // New Wavetable Vec from specs
    fn get_wavetable(specs: &ChipSpecs) -> Vec<f32> {
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
pub(crate) fn note_to_frequency_f64(note: f64) -> f64 {
    libm::pow(2.0, (note - 69.0) / 12.0) * 440.0
}

#[inline(always)]
pub(crate) fn note_to_frequency_f32(note: f32) -> f32 {
    libm::powf(2.0, (note - 69.0) / 12.0) * 440.0
}
