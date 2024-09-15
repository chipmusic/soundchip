use core::ops::RangeInclusive;

#[inline(always)]
pub fn quantize_f32(value: f32, size: f32) -> f32 {
    libm::roundf(value / size) * size
}

pub fn quantize_range_f32(value: f32, steps: u16, range: RangeInclusive<f32>) -> f32 {
    // Fewer than two steps returns zero, useful in setting the pan
    if steps < 2 {
        return 0.0;
    }
    let steps = steps - 1;
    let min = *range.start();
    let max = *range.end();
    let step_size = (max - min) / (steps as f32);
    // Clamp the value to the range [min, max]
    let clamped_value = value.clamp(min, max);
    // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
    let quantized_value = libm::roundf((clamped_value - min) / step_size) * step_size + min;
    // Ensure the result is within the range after quantization
    quantized_value.clamp(min, max)
}

#[inline(always)]
pub fn quantize_f64(value: f64, size: f64) -> f64 {
    libm::round(value / size) * size
}

pub fn quantize_range_f64(value: f64, steps: u16, range: RangeInclusive<f64>) -> f64 {
    // Fewer than two steps returns zero, useful in setting the pan
    if steps < 2 {
        return 0.0;
    }
    let steps = steps - 1;
    let min = *range.start();
    let max = *range.end();
    let step_size = (max - min) / (steps as f64);
    // Clamp the value to the range [min, max]
    let clamped_value = value.clamp(min, max);
    // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
    let quantized_value = libm::round((clamped_value - min) / step_size) * step_size + min;
    // Ensure the result is within the range after quantization
    quantized_value.clamp(min, max)
}
