/// A point in an envelope, with its associated time and value.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Knot {
    pub time: f32,
    pub value: f32, // TODO: If16 here
    pub interpolation: Interpolation,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Interpolation {
    #[default]
    Linear,
    Step,
}

/// An envelope knot in the range of -1.0 to 1.0.
impl Knot {
    pub fn new(time: f32, value: f32) -> Self {
        Self {
            time,
            value,
            ..Default::default()
        }
    }

    pub fn offset(self, offset: f32) -> Self {
        Self {
            value: (self.value + offset),
            ..self
        }
    }

    pub fn scale_value(self, factor: f32) -> Self {
        Self {
            value: (self.value * factor),
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
