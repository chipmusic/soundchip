use crate::prelude::*;
use crate::presets::*;

pub const SPEC_WAVE_FLAT:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_SIGNED_ZERO),
    sample_count: 8,
    use_loop: true,
    steps: Some(0),
};

pub const SPEC_WAVE_CLEAN:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_TRIANGLE),
    sample_count: 256,
    use_loop: true,
    steps: Some(256),
};

pub const SPEC_WAVE_PSG:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_SQUARE),
    sample_count: 8,
    use_loop: true,
    steps: Some(2),
};

pub const SPEC_WAVE_SCC:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_TRIANGLE),
    sample_count: 32,
    use_loop: true,
    steps: Some(256),
};

pub const SPEC_WAVE_PCE:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_TRIANGLE),
    sample_count: 32,
    use_loop: true,
    steps: Some(32),
};

pub const SPEC_WAVE_NES_SQUARE:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_SQUARE),
    sample_count: 8,
    use_loop: true,
    steps: Some(2),
};

pub const SPEC_WAVE_NES_TRIANGLE:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_TRIANGLE),
    sample_count: 32,
    use_loop: true,
    steps: Some(16),
};

/// Needs testing! use_loop doesn't do anything yet.
/// May need a frequency multiplier to "stretch" the sample when playing C4?
pub const SPEC_WAVE_NES_DMC:SpecsWavetable = SpecsWavetable {
    default_waveform: Some(KNOTS_WAVE_TRIANGLE),
    sample_count: 256,
    use_loop: false,
    // In reality, DPCM meant any sample had to be +1 step or -1 step, never the same.
    steps: Some(16),
};
