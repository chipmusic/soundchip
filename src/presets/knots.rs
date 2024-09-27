use crate::prelude::*;

pub const KNOTS_FLAT:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_VOL_SAWTOOTH:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.5, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_VOL_PIANO:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.075, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:0.1, value:0.5, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_WAVE_SQUARE:&[Knot<f32>] = &[
    Knot{time:0.0, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.5, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.501, value:-1.0, interpolation:Interpolation::Linear},
];

pub const KNOTS_WAVE_TRIANGLE:&[Knot<f32>] = &[
    Knot{time:0.0, value:0.0, interpolation:Interpolation::Linear},
    Knot{time:0.25, value:1.0, interpolation:Interpolation::Linear},
    Knot{time:0.75, value:-1.0, interpolation:Interpolation::Linear},
    Knot{time:1.0, value:0.0, interpolation:Interpolation::Linear},
];
