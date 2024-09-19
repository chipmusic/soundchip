//! A few constant presets.
use crate::prelude::{Envelope, EnvelopeState, Knot, SpecsTremolo, SpecsVibratto};

pub const ENVELOPE_PIANO:Envelope = Envelope{
    attack: Knot { time: 0.0, value: 1.0 },
    decay: Knot { time: 0.1, value: 0.5 },
    sustain: Knot { time: 0.3, value: 0.2 },
    release: Knot { time: 1.0, value: 0.0 },
    state: EnvelopeState::Idle,
};

pub const ENVELOPE_LINEAR:Envelope = Envelope{
    attack: Knot { time: 0.0, value: 1.0 },
    decay: Knot { time: 0.25, value: 0.75 },
    sustain: Knot { time: 0.5, value: 0.5 },
    release: Knot { time: 1.0, value: 0.0 },
    state: EnvelopeState::Idle,
};


// Amplitude 1.0 is one octave up or down, 2.0 is two octaves, etc.
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

pub const TREMOLO_SUBTLE:SpecsTremolo = SpecsTremolo{
    steps: Some(4),
    amplitude: 0.1,
    frequency: 7.5,
};

pub const TREMOLO_INTENSE:SpecsTremolo = SpecsTremolo{
    steps: Some(4),
    amplitude: 0.2,
    frequency: 15.0,
};
