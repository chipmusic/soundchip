use crate::prelude::*;
use Interpolation::*;

// Constant zero
pub const KNOTS_FLAT_ZERO:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Linear},
    Knot{time:1.0, value:0.0, interpolation:Linear},
];

// Constant one
pub const KNOTS_FLAT_ONE:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Step},
    Knot{time:1.0, value:1.0, interpolation:Step},
];

// Volume
pub const KNOTS_VOL_DOWN:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Linear},
    Knot{time:0.5, value:0.5, interpolation:Linear},
    Knot{time:1.0, value:0.0, interpolation:Linear},
];

pub const KNOTS_VOL_SQUARE:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Step},
    // Knot{time:0.5, value:0.0, interpolation:Step},
    Knot{time:1.0, value:0.0, interpolation:Step},
];

pub const KNOTS_VOL_PIANO:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Linear},
    Knot{time:0.075, value:0.5, interpolation:Linear},
    Knot{time:0.1, value:0.5, interpolation:Linear},
    Knot{time:1.0, value:0.0, interpolation:Linear},
];

// Pitch
pub const KNOTS_PITCH_DOWN:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Linear},
    Knot{time:0.5, value:-0.5, interpolation:Linear},
    Knot{time:1.0, value:-1.0, interpolation:Linear},
];

pub const KNOTS_PITCH_UP:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Linear},
    Knot{time:0.5, value:0.5, interpolation:Linear},
    Knot{time:1.0, value:1.0, interpolation:Linear},
];

// Wavetables
pub const KNOTS_WAVE_SQUARE:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Step},
    Knot{time:0.5, value:-1.0, interpolation:Step},
    Knot{time:1.0, value:-1.0, interpolation:Step},
];

pub const KNOTS_WAVE_TRIANGLE:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Linear},
    Knot{time:0.25, value:1.0, interpolation:Linear},
    Knot{time:0.75, value:-1.0, interpolation:Linear},
    Knot{time:1.0, value:0.0, interpolation:Linear},
];

pub const KNOTS_WAVE_SAWTOOTH:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Linear},
    Knot{time:1.0, value:-1.0, interpolation:Linear},
];
