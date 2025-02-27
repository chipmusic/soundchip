use crate::prelude::*;

pub const TREMOLO_SUBTLE:Tremolo = Tremolo{
    steps: None,
    amplitude: 0.1,
    frequency: 7.5,
};

pub const TREMOLO_INTENSE:Tremolo = Tremolo{
    steps: None,
    amplitude: 0.2,
    frequency: 15.0,
};

pub const TREMOLO_ROUGH:Tremolo = Tremolo{
    steps: None,
    amplitude: 0.5,
    frequency: 15.0,
};
