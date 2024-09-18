use core::ops::RangeInclusive;

/// The processing specs for pitch values. Usually Tone and Noise will have different pitch specs.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecsPitch {
    /// Fixed multiplier.
    pub multiplier: f32,
    /// Optional clamp range.
    pub range: Option<RangeInclusive<f32>>,
    /// Optional quantization steps within the provided range.
    /// Has no effect without a valid range.
    pub steps: Option<u16>,
}

impl Default for SpecsPitch {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            range: Some(16.35 ..= 16744.04),
            steps: Some(4096),
        }
    }
}

impl SpecsPitch {

    pub fn psg() -> Self {
        Self {
            multiplier: 1.0,
            range: Some(16.35 ..= 16744.04),
            steps: Some(4096),
        }
    }

    pub fn scc() -> Self {
        Self {
            multiplier: 1.0,
            range: Some(16.35 ..= 16744.04),
            steps: Some(4096),
        }
    }

}
