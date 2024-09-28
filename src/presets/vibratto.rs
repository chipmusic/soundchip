use crate::prelude::*;

pub const VIBRATTO_SUBTLE:Vibratto = Vibratto{
    steps: Some(16),
    amplitude: 1.0 / 48.0,
    frequency: 6.0,
};

pub const VIBRATTO_INTENSE:Vibratto = Vibratto{
    steps: Some(16),
    amplitude: 1.0 / 12.0,
    frequency: 10.0,
};
