use core::ops::RangeInclusive;

// use crate::quantize_f32;

// use crate::quantize;

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

    // // Only used in testing
    // pub fn get(&self, value:f32) -> f32 {
    //     // println!("get from {:?}", self);
    //     let mut value = value;
    //     // Optional Apply max range
    //     if let Some(range) = &self.range {
    //         let min = *range.start();
    //         let max = *range.end();
    //         value = value.clamp(min, max);
    //         // println!("clamping to {:?}", range);
    //         // Optional Quantize
    //         if let Some(steps) = self.steps {
    //             let range = max - min;
    //             let size = range / steps ;
    //             value = quantize_f32(value, size);
    //             // println!("quantizing to {size}");
    //         }
    //     }
    //     value * self.multiplier
    // }
}
