use crate::prelude::{Envelope, Knot, EnvelopeState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresetEnv {
    None,
    Piano,
    LinearDown,
    LinearUp,
}

impl PresetEnv {

    pub fn get_envelope(&self) -> Option<Envelope> {
        match self {
            PresetEnv::None => {
                None
            }
            PresetEnv::Piano => {Some(
                Envelope{
                    attack: Knot { time: 0.0, value: 1.0 },
                    decay: Knot { time: 0.1, value: 0.5 },
                    sustain: Knot { time: 0.3, value: 0.2 },
                    release: Knot { time: 1.0, value: 0.0 },
                    state: EnvelopeState::Idle,
                }
            )},
            PresetEnv::LinearDown => {Some(
                Envelope{
                    attack: Knot { time: 0.0, value: 1.0 },
                    decay: Knot { time: 0.25, value: 0.75 },
                    sustain: Knot { time: 0.5, value: 0.5 },
                    release: Knot { time: 1.0, value: 0.0 },
                    state: EnvelopeState::Idle,
                }
            )},
            PresetEnv::LinearUp => {Some(
                Envelope{
                    attack: Knot { time: 0.0, value: 0.0 },
                    decay: Knot { time: 0.25, value: 0.25 },
                    sustain: Knot { time: 0.5, value: 0.5 },
                    release: Knot { time: 1.0, value: 1.0 },
                    state: EnvelopeState::Idle,
                }
            )},
        }
    }
}
