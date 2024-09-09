use std::ops::Range;

use crate::quantize;

/// The processing specs for pitch values. Usually Tone and Noise will have different pitch specs.
#[derive(Debug, Clone, PartialEq)]
pub struct PitchSpecs {
    /// Fixed multiplier.
    pub multiplier: f32,
    /// Optional clamp range.
    pub range: Option<Range<f32>>,
    /// Optional quantization steps within the provided range.
    /// Has no effect without a valid range.
    pub steps: Option<u16>,
}

impl Default for PitchSpecs {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            range: Some(16.35 .. 16744.04),
            steps: Some(4096),
        }
    }
}

impl PitchSpecs {

    pub fn get(&self, value:f32) -> f32 {
        // println!("get from {:?}", self);
        let mut value = value;
        // Optional Apply max range
        if let Some(range) = &self.range {
            value = value.clamp(range.start, range.end);
            // println!("clamping to {:?}", range);
            // Optional Quantize
            if let Some(steps) = self.steps {
                let range = range.end - range.start;
                let size = range / steps as f32;
                value = quantize(value, size);
                // println!("quantizing to {size}");
            }
        }
        value * self.multiplier
    }
}
