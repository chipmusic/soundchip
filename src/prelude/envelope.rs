mod knot;
pub use knot::*;

mod state;
pub use state::*;

use crate::{math::lerp, prelude::ChipError};
/// A simple ADSR envelope, with values in the range -1.0 to 1.0. Keep values positive for volume envelopes,
/// but pitch envelopes can have negative values.
#[derive(Debug, Default, Clone)]
pub struct Envelope {
    pub state: EnvelopeState,
    pub(crate) attack: Knot,
    pub(crate) decay: Knot,
    pub(crate) sustain: Knot,
    pub(crate) release: Knot,
}

// TODO: sustain:bool, if true prevents Sustain state to change into Release.
// Maybe an allow_sustain that sets it to true on resetting, and a release() method to set it to false?

impl Envelope {
    pub fn new(attack: Knot, decay: Knot, sustain: Knot, release: Knot) -> Result<Self, ChipError> {
        if attack.value < -1.0 || attack.value > 1.0 {
            return Err(ChipError::InvalidEnvelope);
        };
        if decay.value < -1.0 || decay.value > 1.0 {
            return Err(ChipError::InvalidEnvelope);
        };
        if sustain.value < -1.0 || sustain.value > 1.0 {
            return Err(ChipError::InvalidEnvelope);
        };
        if release.value < -1.0 || release.value > 1.0 {
            return Err(ChipError::InvalidEnvelope);
        };
        Ok(Self {
            state: EnvelopeState::Idle,
            attack,
            decay,
            sustain,
            release,
        })
    }

    pub fn offset(self, offset: f32) -> Self {
        Self {
            attack: self.attack.offset(offset),
            decay: self.decay.offset(offset),
            sustain: self.sustain.offset(offset),
            release: self.release.offset(offset),
            state: self.state,
        }
    }

    pub fn scale_values(self, factor: f32) -> Self {
        Self {
            attack: self.attack.scale_value(factor),
            decay: self.decay.scale_value(factor),
            sustain: self.sustain.scale_value(factor),
            release: self.release.scale_value(factor),
            state: self.state,
        }
    }

    pub fn scale_time(self, factor: f32) -> Self {
        Self {
            attack: self.attack.scale_time(factor),
            decay: self.decay.scale_time(factor),
            sustain: self.sustain.scale_time(factor),
            release: self.release.scale_time(factor),
            state: self.state,
        }
    }

    pub fn reset(&mut self) {
        self.state = EnvelopeState::Attack;
    }

    pub fn set_idle(&mut self) {
        self.state = EnvelopeState::Idle
    }

    pub fn process(&mut self, time: f32) -> f32 {
        match self.state {
            EnvelopeState::Attack => {
                if time >= self.attack.time {
                    self.state = EnvelopeState::Decay;
                    // println!("time: {:.1}, New state: {:?}", time, self.state);
                    return self.process(time);
                }
                let x = time / self.attack.time;
                self.attack.value * x
            }
            EnvelopeState::Decay => {
                if time >= self.decay.time {
                    self.state = EnvelopeState::Sustain;
                    // println!("time: {:.1}, New state: {:?}", time, self.state);
                    return self.process(time);
                }
                let time = time - self.attack.time;
                let decay_time = self.decay.time - self.attack.time;
                let t = time / decay_time;
                lerp(self.attack.value, self.decay.value, t)
            }
            EnvelopeState::Sustain => {
                if time >= self.sustain.time {
                    self.state = EnvelopeState::Release;
                    // println!("time: {:.1}, New state: {:?}", time, self.state);
                    return self.process(time);
                }
                let time = time - self.decay.time;
                let sustain_time = self.sustain.time - self.decay.time;
                let t = time / sustain_time;
                lerp(self.decay.value, self.sustain.value, t)
            }
            EnvelopeState::Release => {
                if time >= self.release.time {
                    self.state = EnvelopeState::Idle;
                    // println!("time: {:.1}, New state: {:?}", time, self.state);
                    return self.process(time);
                }
                let time = time - self.sustain.time;
                let release_time = self.release.time - self.sustain.time;
                let t = time / release_time;
                lerp(self.sustain.value, self.release.value, t)
            }
            EnvelopeState::Idle => self.release.value,
        }
    }
}
