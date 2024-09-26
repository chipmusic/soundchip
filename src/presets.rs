//! A few constant presets.
use crate::prelude::{Interpolation, Knot, SpecsTremolo, SpecsVibratto};

// Knots
pub const KNOTS_SAWTOOTH:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.5, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_CUTOFF:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.5, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.501, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_TRIANGLE:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Interpolation::Linear},
    Knot{time:0.25, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.75, value:-1.0, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_PIANO:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.075, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:0.1, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

// Vibratto
pub const VIBRATTO_SUBTLE:SpecsVibratto = SpecsVibratto{
    steps: Some(16),
    amplitude: 1.0 / 48.0,
    frequency: 6.0,
};

pub const VIBRATTO_INTENSE:SpecsVibratto = SpecsVibratto{
    steps: Some(16),
    amplitude: 1.0 / 12.0,
    frequency: 10.0,
};

// Tremolo
pub const TREMOLO_SUBTLE:SpecsTremolo = SpecsTremolo{
    steps: None,
    amplitude: 0.1,
    frequency: 7.5,
};

pub const TREMOLO_INTENSE:SpecsTremolo = SpecsTremolo{
    steps: None,
    amplitude: 0.2,
    frequency: 15.0,
};
