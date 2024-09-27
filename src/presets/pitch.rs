use crate::prelude::*;

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
