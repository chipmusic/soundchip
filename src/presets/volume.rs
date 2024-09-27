use crate::prelude::*;

pub const SPEC_VOLUME_CLEAN:SpecsVolume = SpecsVolume {
    steps: None,
    attenuation: 0.0,
    exponent: 2.5,
    gain: 1.0,
    clip_negative_values: false,
};

pub const SPEC_VOLUME_PSG:SpecsVolume = SpecsVolume {
    steps: Some(16),
    attenuation: 0.001,
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
