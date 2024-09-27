use crate::prelude::*;

// Noise
const FREQ_C0:f32 = 16.35;
const FREQ_C1:f32 = 32.7;
const FREQ_C2:f32 = 35.4;
const FREQ_C3:f32 = 130.81;
// const FREQ_C7:f32 = 2093.0;
const FREQ_C8:f32 = 4186.0;
const FREQ_C9:f32 = 8372.0;
// const FREQ_DS4:f32 = 311.13;
const FREQ_C10:f32 = 16744.04;
const FREQ_GS5:f32 = 830.61;

pub const SPEC_NOISE_MSX: SpecsNoise = SpecsNoise::Random {
    volume_steps: 2,
    pitch: SpecsPitch {
        multiplier: 55.0,
        steps: Some(32),
        range: Some(FREQ_C3 ..= FREQ_GS5),
    },
};

pub const SPEC_NOISE_PCE: SpecsNoise = SpecsNoise::Random {
    volume_steps: 2,
    pitch: SpecsPitch {
        multiplier: 55.0,
        steps: Some(4096),
        range: Some(FREQ_C0 ..= FREQ_C10),
    },
};

/// Placeholder, needs better values
pub const SPEC_NOISE_POKEY: SpecsNoise = SpecsNoise::Melodic {
    lfsr_length: 5,
    volume_steps: 2,
    pitch: SpecsPitch {
        multiplier: 5.0,
        steps: Some(128),
        range: Some(FREQ_C1 ..= FREQ_C9),
    },
};

/// Placeholder, needs better values. Should be 16 steps, I'm cheating a bit here to be
/// more flexible with pitches.
pub const SPEC_NOISE_NES: SpecsNoise = SpecsNoise::Random {
    volume_steps: 2,
    pitch: SpecsPitch {
        multiplier: 15.46,
        steps: Some(32),
        range: Some(FREQ_C2 ..= FREQ_C8),
    },
};

/// Placeholder, needs better values. Should be 16 steps, I'm cheating a bit here to be
/// more flexible with pitches.
pub const SPEC_NOISE_NES_MELODIC: SpecsNoise = SpecsNoise::Melodic {
    lfsr_length: 5,
    volume_steps: 2,
    pitch: SpecsPitch {
        multiplier: 5.0,
        steps: Some(32),
        range: Some(FREQ_C2 ..= FREQ_C8),
    },
};
