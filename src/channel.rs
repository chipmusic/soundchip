use crate::{quantize, quantize_full_range, ChipError, ChipSpecs, Noise, Rng, Sample};
use alloc::vec::Vec;
use core::f32::consts::TAU;

/// A single sound channel with configurable properties.
pub struct Channel {
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    pub loop_sample: bool,
    // Internal state. All timing values are f64, sample values are f32
    specs: ChipSpecs,
    playing: bool,
    noise_on: bool,
    wavetable: Vec<f32>,
    period: f64,
    time: f64,
    output: f32,
    volume: f32,
    pan: f32,
    octave: i32,
    note: i32,
    left_multiplier: f32,
    right_multiplier: f32,
    last_sample_value: f32,
    last_sample_index: usize,
    // Gets populated automatically with inverted and clamped value from chip specs volume_attenuation
    output_attenuation: f32,
    rng: Rng,
    rng_cache: Vec<f32>,
}

impl Default for Channel {
    fn default() -> Self {
        println!("New default channel");
        let specs = ChipSpecs::default();
        let rng = Self::get_rng(&specs);
        let rng_cache = Self::get_rng_cache(&specs);
        Self {
            // Default sine wave with 16 samples.
            wavetable: (0..16)
                .map(|i| {
                    let a = (i as f32 / 16.0) * TAU;
                    libm::sinf(a)
                })
                .collect(),
            playing: false,
            noise_on: false,
            volume: 1.0,
            pan: 0.0,
            loop_sample: true,
            octave: 4,
            note: 60,
            period: 1.0 / 261.63,
            time: 0.0,
            left_multiplier: 0.5,
            right_multiplier: 0.5,
            output: 0.0,
            output_attenuation: 0.0,
            last_sample_index: 0,
            last_sample_value: 0.0,
            specs,
            rng,
            rng_cache,
        }
    }
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new_psg(allow_noise: bool) -> Self {
        let specs = ChipSpecs {
            sample_steps: 1,
            volume_gain: 1.0,
            volume_attenuation: 0.001,
            noise: if allow_noise {
                Noise::Random { volume_steps: 1, pitch_multiplier: 1.0 }
            } else {
                Noise::None
            },
            prevent_negative_values: true,
            ..Default::default()
        };
        let rng = Self::get_rng(&specs);
        let rng_cache = Self::get_rng_cache(&specs);
        Self {
            specs,
            rng,
            rng_cache,
            ..Default::default()
        }
    }

    /// Creates a new channel configured with a 32 byte wavetable
    pub fn new_scc() -> Self {
        let specs = ChipSpecs {
            sample_steps: 256,
            noise: Noise::None,
            volume_gain: 1.0,
            ..Default::default()
        };
        let rng = Self::get_rng(&specs);
        let rng_cache = Self::get_rng_cache(&specs);
        Self {
            specs,
            rng, rng_cache,
            wavetable: (0..32)
                .map(|i| {
                    let a = (i as f32 / 32.0) * TAU;
                    libm::sinf(a)
                })
                .collect(),
            ..Default::default()
        }
    }

    /// Allows sound generation on this channel.
    pub fn play(&mut self) {
        self.playing = true;
        self.calculate_multipliers();
    }

    /// Stops sound generation on this channel.
    pub fn stop(&mut self) {
        self.playing = false;
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
        self.octave
    }

    /// Current midi note (C4 = 60).
    pub fn note(&self) -> i32 {
        self.note
    }

    /// Current frequency.
    pub fn pitch(&self) -> f32 {
        1.0 / self.period as f32
    }

    /// Current volume. Values above 1.0 may cause clipping.
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

    /// TODO: Better noise Rng per noise settings
    pub fn set_specs(&mut self, specs: ChipSpecs) {
        self.rng = Self::get_rng(&specs);
        self.rng_cache = Self::get_rng_cache(&specs);
        self.specs = specs;
    }

    fn get_rng_cache(specs:&ChipSpecs) -> Vec<f32> {
        match specs.noise {
           Noise::None => vec![],
           Noise::Random { .. } => vec![],
           Noise::Melodic { lfsr_length, volume_steps, ..  } => {
               Rng::as_vec(lfsr_length as u32, 1, volume_steps)
           },
           Noise::WaveTable { .. } => vec![],
       }
    }

    fn get_rng(specs:&ChipSpecs) -> Rng {
         match specs.noise {
            Noise::None => Rng::new(8, 123),
            Noise::Random { .. } => Rng::new(16, 123),
            Noise::Melodic { lfsr_length, .. } => Rng::new(lfsr_length as u32, 1),
            Noise::WaveTable { .. } => Rng::new(8, 123),
        }
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
        self.volume = volume;
        self.calculate_multipliers();
    }

    /// Stereo panning, from left (-1.0) to right (1.0). Centered is zero. Will be quantized per ChipSpecs.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
        self.calculate_multipliers();
    }

    /// Switches channel between tone and noise generation, if specs allow noise.
    pub fn set_noise(&mut self, state: bool) {
        if self.specs.noise != Noise::None {
            self.noise_on = state;
        }
    }

    /// Adjusts internal pitch values to correspond to octave and note ( where C = 0, C# = 1, etc.).
    /// "reset_time" forces the waveform to start from position 0, ignoring previous phase.
    pub fn set_note(&mut self, octave: impl Into<i32>, note: impl Into<i32>, reset_time: bool) {
        // Handle negative values and values beyond range
        self.octave = wrap(octave.into(), 10);
        self.note = wrap(note.into(), 12);
        // MIDI note number, where C4 is 60
        let midi_note_number = (self.octave + 1) * 12 + self.note;
        self.set_midi_note(midi_note_number, reset_time);
    }

    /// Same as set_note, but the notes are an f32 value which allows "in-between" notes, or pitch sliding,
    /// and uses MIDI codes instead of octave and note, i.e. C4 is MIDI code 60.
    pub fn set_midi_note(&mut self, note: impl Into<f64>, reset_time: bool) {
        // cache current phase to re-apply at the end
        let previous_phase = (self.time % self.period as f64) / self.period as f64;
        // Calculate note frequency
        let frequency = libm::pow(2.0, (note.into() - 69.0) / 12.0) * 440.0;
        self.period = 1.0 / frequency;
        // If looping isn't required, ensure sample will be played from beginning.
        // Also, if channel is not playing it means we'll start playing a cycle
        // from 0.0 to avoid clicks.
        self.time = if !self.loop_sample || !self.playing || reset_time {
            0.0
        } else {
            // Adjust time to ensure continuous change (instead of abrupt change)
            previous_phase * self.period
        };
    }

    #[inline(always)]
    /// Returns the current sample and advances the internal timer.
    pub(crate) fn sample(&mut self, delta_time: f64) -> Sample<f32> {
        // Always apply attenuation, so that values always drift to zero
        self.output *= self.output_attenuation;

        // Early return if not playing
        if !self.playing {
            return Sample {
                left: self.output * self.left_multiplier,
                right: self.output * self.right_multiplier,
            };
        }

        // Pre calc noise
        let rng_len = self.rng_cache.len();
        let noise = match self.specs.noise {
            Noise::None => { 0.0 },
            Noise::Random { volume_steps, .. } => {
                quantize_full_range(self.rng.next_f32(), volume_steps)
            },
            Noise::Melodic { pitch_multiplier, .. } => {
                let phase = ((self.time * pitch_multiplier as f64) % self.period) / self.period;
                let index = (phase * rng_len as f64) as usize;
                self.rng_cache[index]
            },
            Noise::WaveTable { .. } => { 0.0 },
        };

        // Determine sample index
        let len = self.wavetable.len();
        let index = if self.loop_sample {
            let phase = (self.time % self.period) / self.period;
            (phase * len as f64) as usize
        } else {
            let phase = (self.time / self.period).clamp(0.0, 1.0);
            ((phase * len as f64) as usize).clamp(0, len - 1)
        };

        // Obtain sample value and set it to output
        if index != self.last_sample_index {
            self.last_sample_index = index;
            // TODO: Optional quantization!
            let value = quantize_full_range(self.wavetable[index] as f32, self.specs.sample_steps);
            // Avoids resetting attenuation if value hasn't changed
            if value != self.last_sample_value {
                if self.specs.prevent_negative_values {
                    self.output = (value + 1.0) / 2.0;
                } else {
                    self.output = value;
                }
                self.last_sample_value = value;
            }
        }

        // Overwrite or mix with noise
        if self.noise_on {
            if self.specs.prevent_negative_values {
                self.output = (noise + 1.0) / 2.0;
            } else {
                self.output = noise;
            }
        }

        // Advance timer
        self.time += delta_time;

        Sample {
            left: self.output * self.left_multiplier,
            right: self.output * self.right_multiplier,
        }
    }

    #[inline(always)]
    // Must be called after setting volume or pan.
    // Used to pre-calculate as many values as possible instead of doing it per sample, since
    // this function is called much less frequently (by orders of magnitude)
    pub(crate) fn calculate_multipliers(&mut self) {
        // Pre calculate this so we don't do it on every sample
        self.output_attenuation = 1.0 - self.specs.volume_attenuation.clamp(0.0, 1.0);
        // "powf" only gives the intended result in the 0 to 1 range, so we only apply
        // the chip's gain after the pow function.
        let volume = libm::powf(
            quantize(self.volume, self.specs.volume_steps),
            self.specs.volume_exponent,
        ) * self.specs.volume_gain;
        let pan = quantize(self.pan, self.specs.pan_steps);
        self.left_multiplier = ((pan - 1.0) / -2.0) * volume;
        self.right_multiplier = ((pan + 1.0) / 2.0) * volume;
    }
}

fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}
