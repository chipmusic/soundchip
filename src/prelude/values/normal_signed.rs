use crate::prelude::KnotValue;
use core::fmt::{Display, Formatter};

/// Stored internally as i16, allows converting to/from a -1.0 to 1.0 f32 range.
#[derive(Debug, Clone, Copy, Default)]
pub struct NormalSigned(i16);

const MAX: f32 = i16::MAX as f32;

/// Returns an f32 value between -1.0 and 1.0 (inclusive).
impl Into<f32> for NormalSigned {
    fn into(self) -> f32 {
        self.0 as f32 / MAX
    }
}

/// Will clamp values outside valid range of -1.0 to 1.0.
impl From<f32> for NormalSigned {
    fn from(value: f32) -> Self {
        let clamped = (value.clamp(-1.0, 1.0) * MAX) as i16;
        Self(clamped)
    }
}

impl Display for NormalSigned {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let n: f32 = self.0.into();
        f.write_fmt(format_args!("{}", n))
    }
}

impl PartialEq for NormalSigned {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl KnotValue for NormalSigned {}


#[test]
fn normal_signed_value_clip() {
    let a = -1.5;
    let a_normal = NormalSigned::from(a);
    let a_converted: f32 = a_normal.into();
    assert_eq!(a_converted, -1.0); // clipped to -1.0

    let a = 1.5;
    let a_normal = NormalSigned::from(a);
    let a_converted: f32 = a_normal.into();
    assert_eq!(a_converted, 1.0); // clipped to 1.0
}

#[test]
// NOT lossless, tiny changes can occur. Less precise than Normal,
// four decimals versus five.
fn normal_signed_value_precision() {
    for n in 0 .. 10 {
        let a = n as f32 / 10.0;
        let a_normal = NormalSigned::from(a);
        let a_converted: f32 = a_normal.into();
        assert!((a_converted - a).abs() < 0.0001);
    }
}
