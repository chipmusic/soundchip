use libm::powf;

use crate::{math::*, prelude::*, presets::*, rng::*, Vec};
use core::f32::consts::TAU;

const FREQ_C4: f32 = 261.63;

/// A single sound channel with configurable properties. The easiest way to create a Channel
/// is using Channel::from(spec), and provide one of the Specs from the "presets" module,
/// or create your own spec from scratch.
pub struct Channel {
    // Wavetable
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    wave_env: Envelope<NormalSigned>,
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
    pub volume_env: Option<Envelope<Normal>>,
    /// Optional volume tremolo. Acts as a secondary envelope subtracted from the regular volume envelope.
    pub tremolo: Option<Tremolo>,
    volume: f32,
    volume_attn: f32,
    // Pitch
    /// Optional pitch envelope. Range -1.0 ..= 1.0 means one octave down or up,
    /// but values can be beyond that range (use "envelope.scale_values(factor)"" to easily change that).
    pub pitch_env: Option<Envelope<f32>>,
    /// Optional pitch vibratto. Acts as a secondary envelope, added to the regular pitch envelope.
    pub vibratto: Option<Vibratto>,
    // Noise
    rng: Rng,
    noise_on: bool,
    noise_period: f32,
    noise_output: f32,
    // State
    specs: SpecsChip,
    pan: NormalSigned,
    midi_note: f32,
    playing: bool,
    left_mult: f32,
    right_mult: f32,
    last_sample_index: usize,
    last_sample_value: f32,
    last_cycle_index: usize,
    // Envelope processing
    env_period: f32,
    last_env: LatestEnvelopes,
    last_env_time: f32,
}

impl From<SpecsChip> for Channel {
    fn from(specs: SpecsChip) -> Self {
        let wave_env = Envelope::from(KNOTS_WAVE_TRIANGLE);
        let mut result = Self {
            // Timing
            phase: 0.0,
            time: 0.0,
            time_env: 0.0,
            time_tone: 0.0,
            time_noise: 0.0,
            period: 1.0 / FREQ_C4,
            // Wavetable
            wavetable: Self::get_wavetable_from_specs(&specs),
            wave_env,
            wave_out: 0.0,
            // Volume
            volume: 1.0,
            volume_env: None,
            volume_attn: 0.0,
            tremolo: None,
            // Pitch
            pitch_env: None,
            // pitch_env_multiplier: 2.0,
            vibratto: None,
            // Noise
            rng: Self::get_rng(&specs),
            noise_on: false,
            noise_period: 0.0,
            noise_output: 0.0,
            // State
            specs,
            pan: NormalSigned::from(0.0),
            midi_note: 60.0,
            playing: false,
            left_mult: 0.5,
            right_mult: 0.5,
            last_sample_index: 0,
            last_sample_value: 0.0,
            last_cycle_index: 0,
            // Envelope processing
            env_period: 0.0,
            last_env_time: 0.0,
            last_env: LatestEnvelopes::default(),
        };
        result.set_note(4, Note::C);
        result.set_volume(1.0);
        result.reset();
        result
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::from(SpecsChip::default())
    }
}

impl Channel {
    /// Allows sound generation on this channel.
    pub fn play(&mut self) {
        self.playing = true;
        self.calculate_multipliers();
    }

    /// Sets all of the channel's relevant properties to match the sound's properties, but
    /// does not change the specs (the only exception is the wavetable envelope,
    /// which can be set by the sound).
    pub fn set_sound(&mut self, sound:&Sound){
        self.set_volume(sound.volume);
        self.set_pitch(sound.pitch);
        self.volume_env = sound.volume_envelope.clone();
        self.pitch_env = sound.pitch_envelope.clone();
        self.vibratto = sound.vibratto;
        self.tremolo = sound.tremolo;
        if let Some(env) = &sound.waveform {
            // Discards result if error, for now. TODO: Return results for fallible functions
            let _ = self.set_wavetable(env);
        }
    }

    /// Stops sound generation on this channel.
    pub fn stop(&mut self) {
        self.playing = false;
        self.reset();
    }

    /// "Releases" all envelopes, allowing them to exit their looping state and reach their end
    pub fn release(&mut self) {
        if let Some(env) = &mut self.volume_env {
            env.release();
        }
        if let Some(env) = &mut self.pitch_env {
            env.release();
        }
    }

    /// The current internal time
    pub fn time(&self) -> f32 {
        self.time
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
        self.pan.into()
    }

    /// Mutable access to the wavetable. Be careful to not set values beyond -1.0 to 1.0.
    pub fn wavetable(&mut self) -> &mut Vec<f32> {
        &mut self.wavetable
    }

    /// Resets al internal timers (tone, noise, envelopes)
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.time_tone = 0.0;
        self.time_noise = 0.0;
        self.last_cycle_index = 0;
        self.reset_envelopes();
    }

    /// Resets just the envelope timer.
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

    /// Reconfigures all internals according to new specs
    pub fn set_specs(&mut self, specs: SpecsChip) {
        self.rng = Self::get_rng(&specs);
        self.wavetable = Self::get_wavetable_from_specs(&specs);
        self.specs = specs;
    }

    /// Generates wavetable samples from an envelope
    /// TODO: Normalize time range.
    pub fn set_wavetable(&mut self, wave: &Envelope<NormalSigned>) -> Result<(), ChipError> {
        self.wave_env = wave.clone();
        self.wavetable = Self::get_wavetable(&self.specs, &self.wave_env);
        Ok(())
    }

    /// Directly sets the wavetable from f32 values, ensuring -1.0 to 1.0 range.
    /// Will return an error if values are invalid.
    pub fn set_wavetable_raw(&mut self, table: &[f32]) -> Result<(), ChipError> {
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
        self.pan = pan.into();
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
        let frequency = note_to_frequency(note);
        self.set_pitch(frequency);
    }

    /// Directly set the channel's frequency.
    pub fn set_pitch(&mut self, frequency:f32){
        self.midi_note = frequency_to_note(frequency);
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
        self.last_env = self.process_envelopes();
    }

    fn process_envelopes(&mut self) -> LatestEnvelopes {
        // Adjust volume with envelope
        let mut volume_env = if let Some(env) = &mut self.volume_env {
            env.peek(self.time_env)
        } else {
            1.0
        };

        // Apply tremolo
        if let Some(tremolo) = &self.tremolo {
            let sine = libm::sinf(self.time * TAU * tremolo.frequency);
            let quant = if let Some(steps) = tremolo.steps {
                quantize_range(sine, steps, -1.0..=1.0)
            } else {
                sine
            };
            let normalized = ((quant / 2.0) + 0.5) * tremolo.amplitude;
            volume_env = (volume_env - normalized).clamp(0.0, 1.0);
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

        // Pitch envelope
        let mut pitch_change = if let Some(env) = &mut self.pitch_env {
            env.peek(self.time_env)
        } else {
            0.0
        };

        // Apply vibratto
        if let Some(vibratto) = &self.vibratto {
            let sine = libm::sinf(self.time * TAU * vibratto.frequency);
            let quant = if let Some(steps) = vibratto.steps {
                quantize_range(sine, steps, -1.0..=1.0)
            } else {
                sine
            };
            pitch_change = pitch_change + (quant * vibratto.amplitude);
        };

        // Acquire optionally quantized tone period and noise period with pitch change
        let tone_period = (self.period / self.specs.pitch.multiplier) * powf(2.0, -pitch_change);
        let tone_period = if let Some(steps) = self.specs.pitch.steps {
            if let Some(range) = &self.specs.pitch.range {
                // TODO: This needs optimization...
                let freq = 1.0 / tone_period;
                1.0 / quantize_range(freq, steps, (*range).clone())
            } else {
                tone_period
            }
        } else {
            tone_period
        };

        let noise_period = self.noise_period * powf(2.0, -pitch_change);

        // Timing adjust to preserve phase
        self.time_tone = self.phase * tone_period;
        self.last_env_time = self.time;

        // Return
        LatestEnvelopes {
            tone_period,
            noise_period,
            volume,
        }
    }

    #[inline(always)]
    /// Returns the current sample and peeks the internal timer.
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
        let process_envelopes_now = if self.specs.envelope_rate.is_some() {
            // If there's an envelope rate, calculate envelopes when needed.
            if (self.time - self.last_env_time >= self.env_period) || self.time == 0.0 {
                true
            } else {
                false
            }
        } else {
            // If no envelope rate, always calculate envelopes
            true
        };

        // Generate noise level, will be mixed later
        if self.noise_on {
            self.noise_output = match self.specs.noise {
                SpecsNoise::None => 0.0,
                SpecsNoise::Random { volume_steps, .. } | SpecsNoise::Melodic { volume_steps, .. } => {
                    if process_envelopes_now {
                        self.last_env = self.process_envelopes();
                    }
                    if self.time_noise >= self.last_env.noise_period {
                        self.time_noise = 0.0;
                        (quantize_range(self.rng.next_f32(), volume_steps as u16, 0.0..=1.0) * 2.0) - 1.0
                    } else {
                        self.noise_output
                    }
                }
                SpecsNoise::WaveTable { .. } => 0.0,
            };
        }

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
                // Prevents sampling envelope in the middle of a wave cycle
                let cycle_index = (self.time_tone as f64 / self.last_env.tone_period as f64) as usize;
                if cycle_index != self.last_cycle_index {
                    self.last_cycle_index = cycle_index;
                    if process_envelopes_now {
                        self.last_env = self.process_envelopes();
                    }
                }
                self.wave_out = value;
                self.last_sample_value = value;
            }
        }

        // adjust timers
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
        let output = if self.specs.volume.clip_negative_values {
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
            quantize_range(self.pan.into(), pan_steps, -1.0..=1.0)
        } else {
            self.pan.into()
        };
        // Is applying gain to the pan OK? Needs testing
        self.left_mult = ((pan - 1.0) / -2.0) * self.specs.volume.gain;
        self.right_mult = ((pan + 1.0) / 2.0) * self.specs.volume.gain;
        // Envelope period
        if let Some(env_freq) = self.specs.envelope_rate {
            self.env_period = 1.0 / env_freq;
        }
    }

    // New Rng from specs
    fn get_rng(specs: &SpecsChip) -> Rng {
        match specs.noise {
            SpecsNoise::None | SpecsNoise::Random { .. } | SpecsNoise::WaveTable { .. } => {
                Rng::new(15, 1)
            }
            SpecsNoise::Melodic { lfsr_length, .. } => Rng::new(lfsr_length as u32, 1),
        }
    }

    // New Wavetable Vec from specs
    fn get_wavetable_from_specs(specs: &SpecsChip) -> Vec<f32> {
        let mut envelope: Envelope<NormalSigned> =
            if let Some(knots) = specs.wavetable.default_waveform {
                knots.into()
            } else {
                KNOTS_WAVE_TRIANGLE.into()
            };
        (0..specs.wavetable.sample_count)
            .map(|i| {
                // Default sine wave
                let t = i as f32 / specs.wavetable.sample_count as f32;
                // libm::sinf(t * TAU)
                envelope.peek(t).into()
            })
            .collect()
    }

    // New Wavetable Vec
    fn get_wavetable(specs: &SpecsChip, envelope:&Envelope<NormalSigned>) -> Vec<f32> {
        let mut envelope = envelope.clone();
        (0..specs.wavetable.sample_count)
            .map(|i| {
                // Default sine wave
                let t = i as f32 / specs.wavetable.sample_count as f32;
                // libm::sinf(t * TAU)
                envelope.peek(t).into()
            })
            .collect()
    }
}

#[derive(Default)]
struct LatestEnvelopes {
    volume: f32,
    tone_period: f32,
    noise_period: f32,
}
