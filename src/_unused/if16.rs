use super::ChipError;

pub struct If16(i16);

const MAX:f32 = i16::MAX as f32;

impl TryFrom<f32> for If16 {
    type Error = ChipError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 1.0 || value < -1.0 {
            Err(ChipError::InvalidIf16Range)
        } else{
            Ok(If16((value * MAX) as i16))
        }
    }
}

impl Into<f32> for If16 {
    fn into(self) -> f32 {
        self.0 as f32 / MAX
    }
}


impl If16 {



}
