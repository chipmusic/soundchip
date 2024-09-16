use super::{Envelope, Knot, EnvelopeState};


pub const ENV_PIANO:Envelope = Envelope{
    attack: Knot { time: 0.0, value: 1.0 },
    decay: Knot { time: 0.1, value: 0.5 },
    sustain: Knot { time: 0.3, value: 0.2 },
    release: Knot { time: 0.75, value: 0.0 },
    state: EnvelopeState::Attack,
};


pub const ENV_LINEAR_DECAY:Envelope = Envelope{
    attack: Knot { time: 0.0, value: 1.0 },
    decay: Knot { time: 1.0, value: 0.0 },
    sustain: Knot { time: 1.0, value: 0.0 },
    release: Knot { time: 1.0, value: 0.0 },
    state: EnvelopeState::Attack,
};
