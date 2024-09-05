use crate::{ChipError, ChipSpecs, Note, Sample};
use alloc::vec::Vec;

/// A single sound channel with configurable properties.
pub struct Channel {
    /// The virtual Chip used in this channel
    pub chip: ChipSpecs,
    /// Disables any sound generation.
    pub playing: bool,
    /// Enables and disables sample looping. TODO: Looping strategies, i.e. In and Out points.
    pub loop_sample: bool,
    // Internal state
    output: f32,
    volume: f32,
    pan: f32,
    octave: i32,
    note: i32,
    wavetable: Vec<f32>,
    period: f64,
    time: f64,
    left_multiplier: f32,
    right_multiplier: f32,
    last_sample_index:usize,
    last_sample_value:f32,
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new(
        sample_rate: u32,
        volume_steps: u16,
        sample_steps: u16,
        sample_len: usize,
        allow_noise: bool,
    ) -> Self {
        let half = sample_len / 2;
        let wavetable = (0..sample_len)
            .map(|i| if i < half { 1.0 } else { -1.0 })
            .collect();
        let mut result = Channel {
            chip: ChipSpecs {
                sample_rate,
                volume_steps,
                sample_steps,
                allow_noise,
                ..Default::default()
            },
            playing: false,
            volume: 1.0,
            pan: 0.0,
            wavetable,
            loop_sample: true,
            octave: 4,
            note: 60,
            period: 1.0 / 261.63,
            time: 0.0,
            left_multiplier: 0.5,
            right_multiplier: 0.5,
            output: 0.0,
            last_sample_index: 0,
            last_sample_value: 0.0
        };
        result.set_note(4, Note::C);
        result.calculate_multipliers();
        result
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

    // TODO: Quantize!
    /// Current volume. Values above 1.0 may cause clipping.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    // TODO: Quantize!
    /// Current stereo panning. Zero means centered (mono).
    pub fn pan(&self) -> f32 {
        self.pan
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

    /// Adjusts internal pitch values to correspond to octave and note( where C = 0, C# = 1, etc.)
    pub fn set_note(&mut self, octave: impl Into<i32>, note: impl Into<i32>) {
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
        // Also, if channel is not playing it means we'll start playing a note from 0.0.
        self.time = if !self.loop_sample || !self.playing {
            0.0
        } else {
            // Adjust time to ensure continuous change (instead of abrupt change)
            previous_phase * self.period
        };
    }

    #[inline(always)]
    /// Returns the current sample and advances the internal timer.
    pub(crate) fn sample(&mut self, delta_time: f64) -> Sample<f32> {
        // Apply attenuation before anything else
        self.output *= 1.0 - self.chip.attenuation.clamp(0.0, 1.0);

        // Early return if playing
        if !self.playing {
            return Sample {
                left: self.output * self.left_multiplier,
                right: self.output * self.right_multiplier,
            }
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

        // Obtain sample value and selt it to output
        if index != self.last_sample_index {
            self.last_sample_index = index;
            let value = self.wavetable[index] as f32; // TODO: Quantize!
            // Avoids resetting attenuation if value hasn't changed
            if value != self.last_sample_value {
                self.output = value;
                self.last_sample_value = value;
            }
        }
        // // Debug sine wave
        // let value =  (libm::sin(phase * TAU)) as f32 ;

        Sample {
            left: self.output * self.left_multiplier,
            right: self.output * self.right_multiplier,
        }
    }

    #[inline(always)]
    // Must be called after setting volume or pan, pre-calculates left and right multiplier.
    fn calculate_multipliers(&mut self) {
        self.left_multiplier = ((self.pan - 1.0) / -2.0) * self.volume;
        self.right_multiplier = ((self.pan + 1.0) / 2.0) * self.volume;
    }
}

fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}
