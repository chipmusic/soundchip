
use alloc::vec::Vec;
use crate::{Note, Sample};

// const C4_FREQ: f64 = 261.62558;
// const NOTE_MULTIPLIER: f64 = 1.0594631;
// const PITCH_OFFSET:f64 = 0.059463095;

/// A single sound channel with configurable properties.
pub struct Channel {
    /// Disables any sound generation.
    pub muted: bool,
    /// The output mix rate in Hertz, usually 44100 but depends on your sound playback device.
    pub mix_rate: u32,
    /// The maximum number of volume states the chip can render, i.e. 4 bit volume register = 16 steps.
    pub volume_steps: u16,

    /// Quantizes the stereo pan state, i.e. 4 bit pan register = 16 steps.
    pub pan_steps: u16,
    /// The number of steps per sample. For a PSG with only square waves this number doesn't matter
    /// as long as it's 1 or higher. For a simple wavetable like the SCC this number is 256 (1 byte).
    pub sample_steps: u16,
    /// Number of intermediate steps between note pitches.
    pub pitch_steps: u16,
    /// Wether or not this channel can produce noise. 1980's chips usually had a single channel
    /// capable of generating noise.
    pub allow_noise: bool,

    // Internal state
    volume: f32,
    pan: f32,
    octave: i32,
    note: i32,
    wavetable: Vec<f32>,
    period: f64,
    phase_accumulator: f64,
    // time_per_sample: f64,
    last_sample_time: f64,
    left_multiplier: f32,
    right_multiplier: f32,
}

impl Channel {
    /// Creates a new channel configured with a square wave.
    pub fn new(
        mix_rate: u32,
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
            muted: true,
            mix_rate,
            volume: 1.0,
            volume_steps,
            pan: 0.0,
            pan_steps: 16,
            pitch_steps: 32,
            sample_steps,
            wavetable,
            allow_noise,
            octave: 4,
            note: 0,
            period: 0.0,
            phase_accumulator: 0.0,
            last_sample_time: 0.0,
            left_multiplier: 0.5,
            right_multiplier: 0.5,
        };
        result.set_note(4, Note::C);
        result.calculate_multipliers();
        result
    }

    pub fn octave(&self) -> i32 {
        self.octave
    }

    pub fn note(&self) -> i32 {
        self.note
    }

    pub fn pitch(&self) -> f64 {
        1.0 / self.period
    }

    // TODO: Quantize!
    pub fn volume(&self) -> f32 {
        self.volume
    }

    // TODO: Quantize!
    pub fn pan(&self) -> f32 {
        self.pan
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
        self.octave = wrap(octave.into(), 10);
        self.note = wrap(note.into(), 12);
        // MIDI note number, where C4 is 60
        let midi_note_number = (self.octave + 1) * 12 + self.note;
        let frequency = libm::pow(2.0, (midi_note_number as f64 - 69.0) / 12.0) * 440.0;
        self.period = 1.0 / frequency;
    }

    #[inline(always)]
    pub fn sample(&mut self, time: f64) -> Sample<f32> {
        if self.muted {
            return Sample {
                left: 0.0,
                right: 0.0,
            };
        }
        let delta_time = time - self.last_sample_time;
        self.last_sample_time = time;

        let phase = (self.phase_accumulator % self.period) / self.period;
        self.phase_accumulator += delta_time;

        let index = (phase * self.wavetable.len() as f64) as usize;
        let value = self.wavetable[index] as f32; // TODO: Quantize!

        Sample {
            left: value * self.left_multiplier,
            right: value * self.right_multiplier,
        }
    }

    #[inline(always)]
    fn calculate_multipliers(&mut self) {
        self.left_multiplier = ((self.pan - 1.0) / -2.0) * self.volume;
        self.right_multiplier = ((self.pan + 1.0) / 2.0) * self.volume;
    }
}

fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}
