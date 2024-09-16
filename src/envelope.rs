mod knot;
pub use knot::*;

mod state;
pub use state::*;

pub mod presets;

use crate::lerp;

#[derive(Debug, Default, Clone)]
pub struct Envelope {
    pub attack: Knot,
    pub decay: Knot,
    pub sustain: Knot,
    pub release: Knot,
    pub state: EnvelopeState,
}

impl Envelope {
    // TODO: "new()" function that ensures values are valid and returns a result.
    // i.e. time always progresses or stays the same, values are 0.0 to 1.0,
    // Make all fields private.

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
