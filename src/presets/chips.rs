use crate::prelude::*;
use crate::presets::*;

/// No quantization, which sounds less like a 1980's sound chip and more
/// like a 1990's "music-tracker" or like an FM Synth.
pub const SPEC_CHIP_CLEAN:SpecsChip = SpecsChip {
    envelope_rate: None,
    wavetable: SPEC_WAVE_CLEAN,
    pan: SPEC_PAN_CLEAN,
    pitch: SPEC_PITCH_CLEAN,
    volume: SPEC_VOLUME_CLEAN,
    noise: SPEC_NOISE_POKEY,
};

/// Square wave, no noise;
pub const SPEC_CHIP_PSG:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_PSG,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_PSG,
    noise: SpecsNoise::None,
};

/// Square wave, capable of white noise;
pub const SPEC_CHIP_PSG_NOISE:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_PSG,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_PSG,
    noise: SPEC_NOISE_MSX,
};

/// 32 byte wavetable, 1 byte per sample (32x256).
pub const SPEC_CHIP_SCC:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_SCC,
    pan: SPEC_PAN_STEREO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_PSG,
    noise: SpecsNoise::None,
};

/// 32 samples, 5 bits each (32x32).
pub const SPEC_CHIP_PCE:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_PCE,
    pan: SPEC_PAN_STEREO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_PCE,
    noise: SPEC_NOISE_PCE,
};

/// NES APU Square wave. Adjust the wavetable for duty dycle.;
pub const SPEC_CHIP_NES_SQUARE:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_NES_SQUARE,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES,
    noise: SpecsNoise::None,
};

/// 32 x 16 Triangle wave (as long as the envelope is KNOTS_WAVE_TRIANGLE);
pub const SPEC_CHIP_NES_TRIANGLE:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_NES_TRIANGLE,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES_TRIANGLE,
    noise: SpecsNoise::None,
};

/// NES APU Noise.
pub const SPEC_CHIP_NES_NOISE:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_FLAT,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES,
    noise: SPEC_NOISE_NES
};

/// NES APU Noise with setting #2.
pub const SPEC_CHIP_NES_NOISE_MELODIC:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_FLAT,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES,
    noise: SPEC_NOISE_NES_MELODIC
};

/// NES Wave channel.
pub const SPEC_CHIP_NES_DMC:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_NES_DMC,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_PSG,
    noise: SPEC_NOISE_NES
};
