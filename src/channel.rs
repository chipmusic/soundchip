use crate::{quantize, ChipError, ChipSpecs, Sample};
use alloc::vec::Vec;
use core::f32::consts::TAU;

/// A single sound channel with configurable properties.
pub struct Channel {
    /// The virtual Chip used in this channel
    pub chip: ChipSpecs,
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    pub loop_sample: bool,
    // Internal state
    playing: bool,
    output: f32,
    volume: f32,
    pan: f32,
    octave: i32,
    note: i32,
    wavetable: Vec<f32>,
    period: f64,
    time: f64,
    last_sample_index: usize,
    last_sample_value: f32,
    left_multiplier: f32,
    right_multiplier: f32,
    // Gets populated automatically with inverted and clamped value from chip specs volume_attenuation
    output_attenuation: f32,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            // Default sine wave
            wavetable: (0..16)
                .map(|i| {
                    let a = (i as f32 / 16.0) * TAU;
                    libm::sinf(a)
                })
                .collect(),
            chip: ChipSpecs::default(),
            playing: false,
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
        }
    }
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new_psg(sample_rate: u32, allow_noise: bool) -> Self {
        Self {
            chip: ChipSpecs {
                sample_rate,
                sample_steps: 1,
                volume_gain: 5.0,
                allow_noise,
                prevent_negative_values: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Creates a new channel configured with a 32 byte wavetable
    pub fn new_scc(sample_rate: u32) -> Self {
        Self {
            chip: ChipSpecs {
                sample_rate,
                sample_steps: 256,
                allow_noise: false,
                volume_gain: 10.0,
                ..Default::default()
            },
            wavetable:(0..32)
                .map(|i| {
                    let a = (i as f32 / 32.0) * TAU;
                    libm::sinf(a)
                })
                .collect(),
            ..Default::default()
        }
    }

    pub fn play(&mut self) {
        self.playing = true;
        self.calculate_multipliers();
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.playing
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
    pub fn pitch(&self) -> f64 {
        1.0 / self.period
    }

    /// Current volume. Values above 1.0 may cause clipping.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Current stereo panning. Zero means centered (mono).
    pub fn pan(&self) -> f32 {
        self.pan
    }

    /// Mutable access to the wavetable. Be careful to no set values beyond -1.0 to 1.0.
    pub fn wavetable(&mut self) -> &mut Vec<f32> {
        &mut self.wavetable
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

    /// A value between 0.0 and 1.0.In practice it will be quantized using "volume steps".
    /// Will be quantized per chip settings.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.calculate_multipliers();
    }

    /// Stereo panning. Leave at 0.0 for mono channels. Will be quantized per chip settings.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
        self.calculate_multipliers();
    }

    // TODO: f32 note (for pitch sliding), frequency quantization
    /// Adjusts internal pitch values to correspond to octave and note( where C = 0, C# = 1, etc.)
    pub fn set_note(&mut self, octave: impl Into<i32>, note: impl Into<i32>, reset_time: bool) {
        // cache current phase to re-apply at the end
        let previous_phase = (self.time % self.period) / self.period;
        // Handle negative values and values beyond range
        self.octave = wrap(octave.into(), 10);
        self.note = wrap(note.into(), 12);
        // MIDI note number, where C4 is 60
        let midi_note_number = (self.octave + 1) * 12 + self.note;
        let frequency = libm::pow(2.0, (midi_note_number as f64 - 69.0) / 12.0) * 440.0;
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

        // Determine sample index
        let len = self.wavetable.len();
        let index = if self.loop_sample {
            let phase = (self.time % self.period) / self.period;
            (phase * len as f64) as usize
        } else {
            let phase = (self.time / self.period).clamp(0.0, 1.0);
            ((phase * len as f64) as usize).clamp(0, len - 1)
        };

        // Advance timer
        self.time += delta_time;

        // Obtain sample value and set it to output
        if index != self.last_sample_index {
            self.last_sample_index = index;
            // TODO: Optional quantization!
            let value = quantize(self.wavetable[index] as f32, self.chip.sample_steps);
            // Avoids resetting attenuation if value hasn't changed
            if value != self.last_sample_value {
                if self.chip.prevent_negative_values {
                    self.output = (value + 1.0) / 2.0;
                } else {
                    self.output = value;
                }
                self.last_sample_value = value;
            }
        }

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
        self.output_attenuation = 1.0 - self.chip.volume_attenuation.clamp(0.0, 1.0);
        // "pow" only gives the intended result in the 0 to 1 range, so we only apply
        // the chip's gain after the pow function.
        let volume = libm::powf(
            quantize(self.volume, self.chip.volume_steps),
            self.chip.volume_exponent,
        ) * self.chip.volume_gain;
        let pan = quantize(self.pan, self.chip.pan_steps);
        self.left_multiplier = ((pan - 1.0) / -2.0) * volume;
        self.right_multiplier = ((pan + 1.0) / 2.0) * volume;
        // println!("pan:{}, volume:{}, ({},{})", pan, volume, self.left_multiplier, self.right_multiplier);
    }
}

fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}
