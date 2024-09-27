use crate::prelude::*;

pub const SPEC_PAN_CLEAN: SpecsPan = SpecsPan {
    steps: None,
};

pub const SPEC_PAN_STEREO: SpecsPan = SpecsPan {
    steps: Some(16),
};

pub const SPEC_PAN_MONO: SpecsPan = SpecsPan {
    steps: Some(0),
};
