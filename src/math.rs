use core::ops::RangeInclusive;

#[inline(always)]
pub fn quantize_f64(value:f64, size:f64) -> f64 {
    libm::round(value / size) * size
}

#[inline(always)]
pub fn quantize_f32(value:f32, size:f32) -> f32 {
    libm::roundf(value / size) * size
}

pub fn quantize_range_f64(value: f64, steps: u16, range:Option<RangeInclusive<f64>>) -> f64 {
    // Zero returns zero, useful in setting the pan
    if steps == 0 { return 0.0 }
    // 2 or 1 steps results in perfect square wave
    if steps < 3 {
        return if value > 0.0 { 1.0 } else { -1.0 }
    }
    if let Some(range) = range {
        let min = *range.start();
        let max = *range.end();
        let step_size = (max - min) / (steps as f64);
        // Clamp the value to the range [min, max]
        let clamped_value = value.clamp(min, max);
        // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
        let quantized_value = libm::round((clamped_value - min) / step_size) * step_size + min;
        // Ensure the result is within the range after quantization
        quantized_value.clamp(min, max)
    } else {
        let size = 1.0 / steps as f64;
        quantize_f64(value, size)
    }
}

pub fn quantize_range_f32(value: f32, steps: u16, range:Option<RangeInclusive<f32>>) -> f32 {
    // Zero returns zero, useful in setting the pan
    if steps == 0 { return 0.0 }
    // 2 or 1 steps results in perfect square wave
    if steps < 3 {
        return if value > 0.0 { 1.0 } else { -1.0 }
    }
    if let Some(range) = range {
        let min = *range.start();
        let max = *range.end();
        let step_size = (max - min) / (steps as f32);
        // Clamp the value to the range [min, max]
        let clamped_value = value.clamp(min, max);
        // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
        let quantized_value = libm::roundf((clamped_value - min) / step_size) * step_size + min;
        // Ensure the result is within the range after quantization
        quantized_value.clamp(min, max)
    } else {
        let size = 1.0 / steps as f32;
        quantize_f32(value, size)
    }
}



// #[inline(always)]
// pub fn quantize_f64(value:f64, size:f64) -> f64 {
//     libm::round(value / size) * size
// }

// pub fn quantize_steps_f32(value: f32, steps: u16) -> f32 {
//     if steps == 0 { return 0.0 }
//     if steps < 3 {
//         return if value > 0.0 { 1.0 } else { -1.0 }
//     }
//     // Quantize
//     let size = 1.0 / (steps - 1) as f32;
//     quantize(value, size)
// }

// pub fn quantize_steps_f64(value: f64, steps: u16) -> f64 {
//     if steps == 0 { return 0.0 }
//     if steps < 3 {
//         return if value > 0.0 { 1.0 } else { -1.0 }
//     }
//     // Quantize
//     let size = 1.0 / (steps - 1) as f64;
//     quantize_f64(value, size)
// }
