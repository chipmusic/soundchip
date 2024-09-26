mod pan;
mod chip;
mod pitch;
mod noise;
mod volume;
mod tremolo;
mod vibratto;
mod wavetable;

pub use pan::*;
pub use chip::*;
pub use pitch::*;
pub use noise::*;
pub use volume::*;
pub use tremolo::*;
pub use vibratto::*;
pub use wavetable::*;

// Chips
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

/// 32 x 16 Triangle wave (as long as the envelope is KNOTS_TRIANGLE);
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
    wavetable: SPEC_WAVE_NES_SQUARE,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES,
    noise: SPEC_NOISE_NES
};

/// NES APU Noise with setting #2.
pub const SPEC_CHIP_NES_NOISE_MELODIC:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_NES_SQUARE,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_PSG,
    volume: SPEC_VOLUME_NES,
    noise: SPEC_NOISE_NES_MELODIC
};

/// NES Wave channel.
pub const SPEC_CHIP_NES_DMC:SpecsChip = SpecsChip {
    envelope_rate: Some(60.0),
    wavetable: SPEC_WAVE_NES_SQUARE,
    pan: SPEC_PAN_MONO,
    pitch: SPEC_PITCH_NO_TONE,
    volume: SPEC_VOLUME_PSG,
    noise: SPEC_NOISE_NES
};

// Wavetable
pub const SPEC_WAVE_CLEAN:SpecsWavetable = SpecsWavetable {
    sample_count: 256,
    use_loop: true,
    steps: Some(256),
};

pub const SPEC_WAVE_PSG:SpecsWavetable = SpecsWavetable {
    sample_count: 8,
    use_loop: true,
    steps: Some(2),
};

pub const SPEC_WAVE_SCC:SpecsWavetable = SpecsWavetable {
    sample_count: 32,
    use_loop: true,
    steps: Some(256),
};

pub const SPEC_WAVE_PCE:SpecsWavetable = SpecsWavetable {
    sample_count: 32,
    use_loop: true,
    steps: Some(32),
};

pub const SPEC_WAVE_NES_SQUARE:SpecsWavetable = SpecsWavetable {
    sample_count: 8,
    use_loop: true,
    steps: Some(2),
};

pub const SPEC_WAVE_NES_TRIANGLE:SpecsWavetable = SpecsWavetable {
    sample_count: 32,
    use_loop: true,
    steps: Some(16),
};

/// Needs testing! use_loop doesn't do anything yet.
/// May need a frequency multiplier to "stretch" the sample when playing C4?
pub const SPEC_WAVE_NES_DMC:SpecsWavetable = SpecsWavetable {
    sample_count: 256,
    use_loop: false,
    // In reality, DPCM meant any sample had to be +1 step or -1 step, never the same.
    steps: Some(16),
};

// Pan
pub const SPEC_PAN_CLEAN: SpecsPan = SpecsPan {
    steps: None,
};

pub const SPEC_PAN_STEREO: SpecsPan = SpecsPan {
    steps: Some(16),
};

pub const SPEC_PAN_MONO: SpecsPan = SpecsPan {
    steps: Some(0),
};

// Pitch
pub const SPEC_PITCH_NO_TONE: SpecsPitch = SpecsPitch {
    multiplier: 0.0,
    range: None,
    steps: None,
};

pub const SPEC_PITCH_CLEAN: SpecsPitch = SpecsPitch {
    multiplier: 1.0,
    range: None,
    steps: None,
};

pub const SPEC_PITCH_PSG: SpecsPitch = SpecsPitch {
    multiplier: 1.0,
    range: Some(16.35 ..= 16744.04),
    steps: Some(4096),
};

pub const SPEC_PITCH_SCC:SpecsPitch = SPEC_PITCH_PSG;


// Volume
pub const SPEC_VOLUME_CLEAN:SpecsVolume = SpecsVolume {
    steps: None,
    attenuation: 0.0,
    exponent: 2.5,
    gain: 1.0,
    clip_negative_values: false,
};

pub const SPEC_VOLUME_PSG:SpecsVolume = SpecsVolume {
    steps: Some(16),
    attenuation: 0.0015,
    exponent: 3.0,
    gain: 1.0,
    clip_negative_values: true,
};

pub const SPEC_VOLUME_SCC:SpecsVolume = SpecsVolume {
    steps: Some(16),
    attenuation: 0.0015,
    exponent: 3.0,
    gain: 1.0,
    clip_negative_values: true,
};

pub const SPEC_VOLUME_PCE:SpecsVolume = SpecsVolume {
    steps: Some(16),
    attenuation: 0.001,
    exponent: 3.0,
    gain: 1.0,
    clip_negative_values: false,
};

pub const SPEC_VOLUME_NES:SpecsVolume = SpecsVolume {
    steps: Some(16),
    attenuation: 0.0017,
    exponent: 3.0,
    gain: 1.0,
    clip_negative_values: false,
};

pub const SPEC_VOLUME_NES_TRIANGLE:SpecsVolume = SpecsVolume {
    steps: Some(1),
    attenuation: 0.0017,
    exponent: 3.0,
    gain: 1.0,
    clip_negative_values: false,
};

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
