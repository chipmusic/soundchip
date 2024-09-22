mod knot;
pub use knot::*;

mod state;
pub use state::*;

use crate::math::lerp;

use super::LoopKind;

pub const KNOT_CAPACITY: u8 = 8;

// use crate::{math::lerp, prelude::ChipError};
/// A simple envelope that can be interpolated per knot..
#[derive(Debug, Clone, PartialEq)]
pub struct Envelope {
    pub knots: [Knot; KNOT_CAPACITY as usize],
    pub loop_kind: LoopKind,
    pub release: bool,
    head: usize,
    len: usize,
}

// TODO: sustain:bool, if true prevents Sustain state to change into Release.
// Maybe an allow_sustain that sets it to true on resetting, and a release() method to set it to false?

impl Default for Envelope {
    fn default() -> Self {
        Self {
            knots: core::array::from_fn(|i| {
                let time = i as f32 / (KNOT_CAPACITY - 1) as f32;
                Knot {
                    time,
                    value: 1.0 - time,
                    interpolation: Interpolation::Linear,
                }
            }),
            head: 0,
            release: true,
            loop_kind: LoopKind::None,
            len: KNOT_CAPACITY as usize,
        }
    }
}

impl Envelope {
    pub fn from(source: &[Knot]) -> Self {
        Self {
            knots: core::array::from_fn(|i| {
                if i < source.len() {
                    source[i]
                } else {
                    Knot::default()
                }
            }),
            len: source.len(),
            loop_kind: LoopKind::None,
            release: true,
            head: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn offset(mut self, offset: f32) -> Self {
        for knot in &mut self.knots {
            knot.value += offset
        }
        self
    }

    pub fn scale_values(mut self, factor: f32) -> Self {
        for knot in &mut self.knots {
            knot.value *= factor
        }
        self
    }

    pub fn scale_time(mut self, factor: f32) -> Self {
        for knot in &mut self.knots {
            knot.time *= factor
        }
        self
    }

    pub fn loop_kind(mut self, kind: LoopKind) -> Self {
        self.loop_kind = kind;
        self
    }

    pub fn reset(&mut self) {
        self.head = 0;
    }

    pub fn peek(&mut self, time: f32) -> f32 {
        // println!("peeking t:{}, head:{}", time, self.head);
        let first_knot = self.knots[0];
        let last_knot = self.knots[self.len - 1];
        if time <= first_knot.time {
            return first_knot.value;
        }
        if time == last_knot.time {
            return last_knot.value;
        }
        if time > last_knot.time {
            match self.loop_kind {
                LoopKind::None => return last_knot.value,
                LoopKind::Repeat => {
                    let t = get_loop_position_f32(time, first_knot.time, last_knot.time);
                    self.head = 0;
                    return self.peek_within_time_range(t)
                }
                LoopKind::LoopPoints { .. } => {
                    todo!()
                }
            }
        }
        self.peek_within_time_range(time % last_knot.time)
    }

    fn peek_within_time_range(&mut self, time: f32) -> f32 {
        // println!("Peeking time: {:.2}", time);
        let current = self.knots[self.head]; // head should always be valid
        if self.head + 1 < self.len {
            // If there's a "next" we still haven't reached last knot
            let next = self.knots[self.head + 1];
            let local_time = time - current.time;
            // let local_time = time;
            let next_time = next.time - current.time;
            // Detect head change, recurse (should never recurse more than just once)
            if local_time > next_time {
                // Search for the correct knot range
                let mut low = 0;
                let mut high = self.len - 1;
                while low <= high {
                    self.head = (low + high) / 2;
                    let head_time = self.knots[self.head].time;
                    let next_time = self.knots[self.head+1].time;
                    // println!("Next... trying head={}", self.head);
                    if head_time <= time {
                        if next_time >= time {
                            // Found the time range!
                            break
                        } else {
                            // println!("Growing low...");
                            low += 1;
                        }
                    } else {
                        // println!("Shrinking high...");
                        high -= 1;
                    }
                }
                // Re-run with updated head
                return self.peek_within_time_range(time);
            }
            // Return interpolated value
            let x = local_time / next_time;
            // println!("time:{}, local_time:{}, next_time:{}, x: {}", time, local_time, next_time, x);
            lerp(current.value, next.value, x)
        } else {
            // Should not happen, since time should always be in the correct range!
            // TODO: Change to fail graciously.
            // println!("Oh no! {:#.2?}", self);
            unreachable!();
        }
    }
}



pub fn get_loop_position(input_pos:usize, loop_in:usize, loop_out:usize) ->usize {
    if input_pos > loop_out {
        let diff = input_pos - loop_out - 1;
        let width = loop_out - loop_in + 1;
        return (diff % width) + loop_in;
    }
    input_pos
}

pub fn get_loop_position_f32(input_pos:f32, loop_in:f32, loop_out:f32) ->f32 {
    if input_pos > loop_out {
        let diff = input_pos - loop_out;
        let width = loop_out - loop_in;
        return (diff % width) + loop_in;
    }
    input_pos
}
