mod knot;
use core::cmp::Ordering;

pub use knot::*;

mod tests;

use crate::{
    math::lerp,
    prelude::{KnotValue, LoopKind},
    Vec,
};

const SAFETY_EPSILON: f32 = f32::EPSILON * 2.0;

/// A simple envelope that can be interpolated per knot..
#[derive(Debug, Clone, PartialEq)]
pub struct Envelope<T>
where
    T: KnotValue,
{
    pub knots: Vec<Knot<T>>,
    pub loop_kind: LoopKind,
    release: bool,
    release_time: Option<f32>,
    head: usize,
}

// TODO: sustain:bool, if true prevents Sustain state to change into Release.
// Maybe an allow_sustain that sets it to true on resetting, and a release() method to set it to false?

impl<T> Default for Envelope<T>
where
    T: KnotValue,
{
    fn default() -> Self {
        // Couldn't get "collect()" to work with KnotValue trait.
        let len = 2;
        let mut knots = Vec::new();
        for i in 0..len {
            let time = i as f32 / (len - 1) as f32;
            knots.push(Knot {
                time,
                value: (1.0 - time).into(),
                interpolation: Interpolation::Linear,
            });
        }
        Self {
            knots,
            head: 0,
            release: false,
            release_time: None,
            loop_kind: LoopKind::None,
        }
    }
}

/// Generates a new envelope from a slice of knots. Knots values may be clamped
/// depending the target envelope's Knot's value type.
impl<T> From<&[Knot<f32>]> for Envelope<T>
where
    T: KnotValue,
{
    fn from(source: &[Knot<f32>]) -> Self {
        // // Couldn't get "collect()" to work with KnotValue trait.
        let len = source.len();
        let mut knots = Vec::new();
        for i in 0..len {
            let knot = source[i];
            knots.push(Knot {
                time: knot.time,
                value: knot.value.into(),
                interpolation: knot.interpolation,
            })
        }
        knots.sort_by(|a, b| {
            match a.partial_cmp(b){
                Some(comp) => comp,
                None => Ordering::Equal
            }
        });
        Self {
            knots,
            loop_kind: LoopKind::None,
            release: false,
            release_time: None,
            head: 0,
        }
    }
}

impl<T> Envelope<T>
where
    T: KnotValue,
{
    /// The number of knots.
    pub fn len(&self) -> usize {
        self.knots.len()
    }

    /// Adds an offset all knot values. Resulting values may be clipped
    /// depending on the target envelope's Knot's value type.
    pub fn offset_values(mut self, offset: T) -> Self {
        for knot in &mut self.knots {
            *knot = knot.offset(offset);
        }
        self
    }

    /// Multiplies all knot values by a factor. Resulting values may be clipped
    /// depending on the target envelope's Knot's value type.
    pub fn scale_values(mut self, factor: T) -> Self {
        for knot in &mut self.knots {
            *knot = knot.scale_value(factor);
        }
        self
    }

    /// Multiplies every knot's time by a factor.
    pub fn scale_time(mut self, factor: f32) -> Self {
        for knot in &mut self.knots {
            *knot = knot.scale_time(factor);
        }
        self
    }

    /// Changes the loop kind.
    pub fn set_loop(mut self, kind: LoopKind) -> Self {
        self.loop_kind = kind;
        self
    }

    /// Sorts the knots based on their time.
    pub fn sort_by_time(&mut self) {
        self.knots.sort_by(|a, b| {
            match a.partial_cmp(b){
                Some(comp) => comp,
                None => Ordering::Equal
            }
        });
    }

    /// Resets the internal timing values. Recommended to be always
    /// called when resetting the channel (channel.reset() calls this automatically on
    /// the volume and pitch envelope).
    pub fn reset(&mut self) {
        self.head = 0;
        self.release = false;
        self.release_time = None;
    }

    /// Releases the envelope, if loop kind is set to "LoopPoints". Does nothing otherwise.
    pub fn release(&mut self) {
        // println!("Released");
        self.release = true;
    }

    /// Gets the envelope value at "time". Very efficient If the time increments are small,
    /// will trigger a search for the nearest knots if current state is too
    /// far off from the request time.
    pub fn peek(&mut self, time: f32) -> f32 {
        // println!("peeking t:{}, head:{}, repeat:{:?}", time, self.head, self.loop_kind);
        let first_knot = self.knots[0];
        if time <= first_knot.time {
            return first_knot.value.into();
        }

        let last_knot = self.knots[self.knots.len() - 1];
        match self.loop_kind {
            LoopKind::None => {
                self.peek_without_loop(time, first_knot.time, last_knot.time, last_knot.value)
            }
            LoopKind::Repeat => {
                if time == last_knot.time {
                    return last_knot.value.into();
                }
                if time > last_knot.time {
                    self.head = 0;
                    let normal_t = get_loop_position_f32(time, first_knot.time, last_knot.time);
                    return self.peek_within_time_range(normal_t);
                }
                self.peek_within_time_range(time)
            }
            LoopKind::LoopPoints { loop_in, loop_out } => {
                let knot_in = self.knots.get(loop_in as usize);
                let time_in = if let Some(knot) = knot_in {
                    knot.time
                } else {
                    0.0
                };

                let knot_out = self.knots.get(loop_out as usize);
                let time_out = if let Some(knot) = knot_out {
                    knot.time
                } else {
                    last_knot.time
                };

                if self.release {
                    let local_time = if let Some(released_time) = self.release_time {
                        (time - released_time) + time_in
                    } else {
                        if time < time_in {
                            time
                        } else {
                            // println!("Setting release time to {}", time);
                            self.release_time = Some(time);
                            self.head = loop_in as usize;
                            time_in
                        }
                    };
                    self.peek_without_loop(
                        local_time,
                        first_knot.time,
                        last_knot.time,
                        last_knot.value,
                    )
                } else {
                    // println!("Looping with time_out{}", time_out);
                    let loop_t = get_loop_position_f32(time, time_in, time_out);
                    self.peek_within_time_range(loop_t)
                }
            }
        }
    }

    #[inline(always)]
    fn peek_without_loop(&mut self, time: f32, time_in: f32, time_out: f32, value_out: T) -> f32 {
        // println!("time:{}", time);
        if time >= time_out {
            // println!("time out: {}", time_out);
            return value_out.into();
        }
        let normal_t = get_loop_position_f32(time, time_in, time_out);
        // println!("normal time:{}", normal_t);
        self.peek_within_time_range(normal_t)
    }

    // Requires pre-filtering values outside of envelope time range to work! Will hit
    // an unreachable scope if head == len-1.
    fn peek_within_time_range(&mut self, time: f32) -> f32 {
        // "head" should always be valid
        // println!("Peeking time: {:.3}", time);
        let current = self.knots[self.head];
        if self.head + 1 < self.knots.len() {
            // If there's a "next" we still haven't reached last knot
            let next = self.knots[self.head + 1];
            let local_time = time - current.time;
            // let local_time = time;
            let next_time = next.time - current.time;

            // Detect head change, recurse (should never recurse more than just once)
            if local_time > next_time {
                // Search for the correct knot range
                let mut low = 0;
                let mut high = self.knots.len() - 1;
                while low <= high {
                    self.head = (low + high) / 2;
                    let head_time = self.knots[self.head].time;
                    let next_time = self.knots[self.head + 1].time;
                    // println!("Next... trying head={}", self.head);
                    if head_time <= time {
                        if next_time >= time {
                            // Found the time range!
                            break;
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

// pub(crate) fn get_loop_position(input_pos: usize, loop_in: usize, loop_out: usize) -> usize {
//     if input_pos > loop_out {
//         let diff = input_pos - loop_out - 1;
//         let width = loop_out - loop_in + 1;
//         if width == 0 {
//             return loop_out
//         }
//         return (diff % width) + loop_in;
//     }
//     input_pos
// }

///
pub(crate) fn get_loop_position_f32(input_pos: f32, loop_in: f32, loop_out: f32) -> f32 {
    if input_pos > loop_out {
        let diff = input_pos - loop_out;
        let width = loop_out - loop_in;
        if width < SAFETY_EPSILON {
            return loop_out;
        }
        return (diff % width) + loop_in;
    }
    input_pos
}
