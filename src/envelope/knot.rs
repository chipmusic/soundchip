
#[derive(Debug, Default, Clone, Copy)]
pub struct Knot {
    pub time:f32,
    pub value:f32,
}

/// An envelope knot in the range of -1.0 to 1.0.
impl Knot {

    pub fn offset(self, offset:f32) -> Self {
        Self{
            time: self.time,
            value: (self.value + offset).clamp(-1.0, 1.0)
        }
    }

    pub fn scale(self, factor:f32) -> Self {
        Self{
            time: self.time,
            value: (self.value * factor).clamp(-1.0, 1.0)
        }
    }

}
