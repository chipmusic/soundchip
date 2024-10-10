mod knot;
pub use knot::*;

mod tests;

use core::cmp::Ordering;
use crate::{
    math::lerp,
    prelude::{KnotValue, LoopKind},
    Vec,
};

// use super::Normal;

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
    release_loop_pos: f32,
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
            release_loop_pos: 0.0,
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
        knots.sort_by(|a, b| match a.partial_cmp(b) {
            Some(comp) => comp,
            None => Ordering::Equal,
        });
        Self {
            knots,
            loop_kind: LoopKind::None,
            release: false,
            release_time: None,
            release_loop_pos: 0.0,
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
    pub fn offset_values(mut self, offset: f32) -> Self {
        for knot in &mut self.knots {
            *knot = knot.offset(offset.into());
        }
        self
    }

    /// Multiplies all knot values by a factor. Resulting values may be clipped
    /// depending on the target envelope's Knot's value type.
    pub fn scale_values(mut self, factor: f32) -> Self {
        for knot in &mut self.knots {
            *knot = knot.scale_value(factor.into());
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
        self.knots.sort_by(|a, b| match a.partial_cmp(b) {
            Some(comp) => comp,
            None => Ordering::Equal,
        });
    }

    /// Resets the internal timing values. Recommended to be always
    /// called when resetting the channel (channel.reset() calls this automatically on
    /// the volume and pitch envelope).
    pub fn reset(&mut self) {
        self.head = 0;
        self.release = false;
        self.release_time = None;
        self.release_loop_pos = 0.0;
    }

    /// Releases the envelope, if loop kind is set to "LoopPoints". Does nothing otherwise.
    pub fn release(&mut self) {
        // release_time and release_loop_pos are set when "peeking" the envelope, only after
        // time reaches at least the loop_in point.
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

        let get_loop_time = |loop_in: u8, loop_out: u8| -> (f32, f32) {
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
            (time_in, time_out)
        };

        // Delayed release time - prevents setting the release time until
        // we actually reach the loop_in point
        let mut get_release_time = |time_in: f32, loop_pos:f32| {
            if self.release {
                if self.release_time.is_none() {
                    if time >= time_in {
                        self.release_time = Some(time);
                        self.release_loop_pos = loop_pos;
                    }
                }
            }
        };

        match self.loop_kind {
            LoopKind::None => {
                if time >= last_knot.time {
                    return last_knot.value.into();
                }
                return self.peek_within_time_range(time, 1.0);
            }
            LoopKind::Repeat => {
                if time == last_knot.time {
                    return last_knot.value.into();
                }
                if time > last_knot.time {
                    self.head = 0;
                    let normal_t = get_loop_position_f32(time, first_knot.time, last_knot.time);
                    return self.peek_within_time_range(normal_t, 1.0);
                }
                self.peek_within_time_range(time, 1.0)
            }
            LoopKind::Echo { loop_in, loop_out, decay } => {
                let decay:f32 = decay.into();
                let (time_in, time_out) = get_loop_time(loop_in, loop_out);
                let loop_pos = get_loop_position_f32(time, time_in, time_out);

                get_release_time(time_in, loop_pos);

                if let Some(release_time) = self.release_time {
                    let local_time = self.release_loop_pos + (time - release_time);
                    if local_time > last_knot.time {
                        self.head = 0;
                        let normal_t = get_loop_position_f32(local_time, first_knot.time, last_knot.time);
                        let iteration = get_iteration(local_time, first_knot.time, last_knot.time);
                        let attenuation = 1.0 - (1.0 - (decay / iteration));
                        return self.peek_within_time_range(normal_t, attenuation);
                    }
                    return self.peek_within_time_range(local_time, 1.0);
                } else {
                    self.peek_within_time_range(loop_pos, 1.0)
                }
            }
            LoopKind::LoopPoints { loop_in, loop_out } => {
                let (time_in, time_out) = get_loop_time(loop_in, loop_out);
                let loop_pos = get_loop_position_f32(time, time_in, time_out);

                get_release_time(time_in, loop_pos);

                if let Some(release_time) = self.release_time {
                    let local_time = self.release_loop_pos + (time - release_time);
                    if local_time > last_knot.time {
                        return last_knot.value.into();
                    }
                    return self.peek_within_time_range(local_time, 1.0);
                } else {
                    self.peek_within_time_range(loop_pos, 1.0)
                }
            }
        }
    }

    // Requires pre-filtering values outside of envelope time range to work! Will hit
    // an unreachable scope if head == len-1.
    fn peek_within_time_range(&mut self, time: f32, attenuation: f32) -> f32 {
        let last_index = self.knots.len() - 1;
        // "head" should always be valid!
        if self.head < last_index {
            let mut current = self.knots[self.head];
            let mut next = self.knots[self.head+1];

            if time < current.time || time > next.time {
                // Search if time is outside current knot pair
                loop {
                    if time < current.time {
                        #[cfg(debug_assertions)]{
                            if self.head == 0 {
                                unreachable!()
                            }
                        }
                        self.head -= 1;
                        current = self.knots[self.head];
                        next = self.knots[self.head+1];
                    } else if time > next.time {
                        #[cfg(debug_assertions)]{
                            if self.head == last_index {
                                unreachable!()
                            }
                        }
                        self.head += 1;
                        current = self.knots[self.head];
                        next = self.knots[self.head+1];
                    }
                    if time >= current.time && time <= next.time{
                        break
                    }
                }
            }

            // Return interpolated value
            match current.interpolation {
                Interpolation::Linear => {
                    let local_time = time - current.time;
                    let next_time = next.time - current.time;
                    let x = local_time / next_time;
                    // println!("time:{:.2}, head:{}", time, self.head);
                    lerp(current.value, next.value, x) * attenuation
                }
                Interpolation::Step => {
                    // println!("step");
                    current.value.into() * attenuation
                }
            }
        } else {
            // Should bever be reached, since "time" should always be in the correct range
            // and head never goes outside the valid range.
            #[cfg(debug_assertions)]
            {
                unreachable!();
            }
            #[cfg(not(debug_assertions))]
            {
                0.0
            }
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

pub(crate) fn get_loop_position_f32(t: f32, loop_in: f32, loop_out: f32) -> f32 {
    if t > loop_out {
        let diff = t - loop_out;
        let width = loop_out - loop_in;
        if width < SAFETY_EPSILON {
            return loop_out;
        }
        return (diff % width) + loop_in;
    }
    t
}

fn get_iteration(local_time:f32, first_time: f32, last_time: f32) -> f32 {
    if last_time > first_time {
        let delta = last_time - first_time;
        let iteration = (local_time - first_time) / delta;
        libm::floorf(iteration)
    } else {
        libm::floorf(local_time / last_time)
    }
}
