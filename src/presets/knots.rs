use crate::prelude::*;
use Interpolation::*;

// Constant zero
pub const KNOTS_FLAT_ZERO:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ZERO, interpolation:Linear},
    Knot{time:1.0, value:Normal::ZERO, interpolation:Linear},
];

// Constant one
pub const KNOTS_FLAT_ONE:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ONE, interpolation:Step},
    Knot{time:1.0, value:Normal::ONE, interpolation:Step},
];

// Constant zero
pub const KNOTS_SIGNED_ZERO:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ZERO, interpolation:Linear},
    Knot{time:1.0, value:NormalSigned::ZERO, interpolation:Linear},
];

// Constant one
pub const KNOTS_SIGNED_ONE:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ONE, interpolation:Step},
    Knot{time:1.0, value:NormalSigned::ONE, interpolation:Step},
];

// Volume
#[allow(dead_code)]
pub(crate) const KNOTS_VOL_TEST:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ONE, interpolation:Linear},
    Knot{time:0.25, value:Normal::HALF, interpolation:Linear},
    Knot{time:0.5, value:Normal::THREE_QUARTER, interpolation:Linear},
    Knot{time:0.75, value:Normal::HALF, interpolation:Linear},
    Knot{time:2.0, value:Normal::ZERO, interpolation:Linear},
];

pub const KNOTS_VOL_DOWN:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ONE, interpolation:Linear},
    Knot{time:0.5, value:Normal::HALF, interpolation:Linear},
    Knot{time:1.0, value:Normal::ZERO, interpolation:Linear},
];

pub const KNOTS_VOL_UP:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ZERO, interpolation:Linear},
    Knot{time:0.5, value:Normal::HALF, interpolation:Linear},
    Knot{time:1.0, value:Normal::ONE, interpolation:Linear},
];

pub const KNOTS_VOL_UP_DOWN:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ZERO, interpolation:Linear},
    Knot{time:0.5, value:Normal::ONE, interpolation:Linear},
    Knot{time:1.0, value:Normal::ZERO, interpolation:Linear},
];

pub const KNOTS_VOL_SQUARE:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ONE, interpolation:Step},
    // Knot{time:0.5, value:Normal::ZERO, interpolation:Step},
    Knot{time:1.0, value:Normal::ZERO, interpolation:Step},
];

pub const KNOTS_VOL_PIANO:&[Knot<Normal>] = &[
    Knot{time:0.0, value:Normal::ONE, interpolation:Linear},
    Knot{time:0.075, value:Normal::HALF, interpolation:Linear},
    Knot{time:0.1, value:Normal::HALF, interpolation:Linear},
    Knot{time:1.0, value:Normal::ZERO, interpolation:Linear},
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
pub const KNOTS_WAVE_SQUARE:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ONE, interpolation:Step},
    Knot{time:0.5, value:NormalSigned::NEG_ONE, interpolation:Step},
    Knot{time:1.0, value:NormalSigned::NEG_ONE, interpolation:Step},
];

pub const KNOTS_WAVE_SQUARESAW:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ONE, interpolation:Step},
    Knot{time:0.5, value:NormalSigned::NEG_ONE, interpolation:Linear},
    Knot{time:1.0, value:NormalSigned::ONE, interpolation:Linear},
];

pub const KNOTS_WAVE_TRAPEZOID:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ZERO, interpolation:Linear},
    Knot{time:0.1, value:NormalSigned::ONE, interpolation:Linear},
    Knot{time:0.4, value:NormalSigned::ONE, interpolation:Linear},
    Knot{time:0.6, value:NormalSigned::NEG_ONE, interpolation:Linear},
    Knot{time:0.9, value:NormalSigned::NEG_ONE, interpolation:Linear},
    Knot{time:1.0, value:NormalSigned::ZERO, interpolation:Linear},
];

pub const KNOTS_WAVE_TRIANGLE:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ZERO, interpolation:Linear},
    Knot{time:0.25, value:NormalSigned::ONE, interpolation:Linear},
    Knot{time:0.75, value:NormalSigned::NEG_ONE, interpolation:Linear},
    Knot{time:1.0, value:NormalSigned::ZERO, interpolation:Linear},
];

pub const KNOTS_WAVE_SAWTOOTH:&[Knot<NormalSigned>] = &[
    Knot{time:0.0, value:NormalSigned::ONE, interpolation:Linear},
    Knot{time:1.0, value:NormalSigned::NEG_ONE, interpolation:Linear},
];
