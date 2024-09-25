use std::cmp::Ordering;

use crate::prelude::KnotValue;

/// A point in an envelope, with its associated time and value.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Knot<T>
where T:KnotValue
{
    pub time: f32,
    pub value: T,
    pub interpolation: Interpolation,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Interpolation {
    #[default]
    Linear,
    Step,
}

impl<T> Knot<T>
where T:KnotValue
{
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            ..Default::default()
        }
    }

    pub fn offset(self, offset: T) -> Self {
        let offset:f32 = offset.into();
        let v:f32 = self.value.into();
        let value = T::from(v + offset);    // Will clip to valid range
        Self {
            value,
            ..self
        }
    }

    pub fn scale_value(self, factor: T) -> Self {
        let factor:f32 = factor.into();
        let v:f32 = self.value.into();
        let value = T::from(v * factor);    // Will clip to valid range
        Self {
            value,
            ..self
        }
    }

    pub fn scale_time(self, factor: f32) -> Self {
        Self {
            time: (self.time * factor),
            ..self
        }
    }
}

impl<T> PartialOrd for Knot<T>
where T: KnotValue
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time < other.time {
            Some(Ordering::Less)
        } else if self.time > other.time {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
