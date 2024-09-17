use core::ops::RangeInclusive;
use libm::{round, roundf, sinf};

#[inline(always)]
pub(crate) fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

#[inline(always)]
pub(crate) fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}

#[inline(always)]
pub(crate) fn compress_volume(input_vol:f32, max_vol:f32) -> f32 {
    let mult = core::f32::consts::FRAC_2_PI;
    sinf(input_vol/(max_vol*mult))
}

#[inline(always)]
/// Returns the MIDI note value given an octave (zero to 10) and a note (zero to 11).
pub fn get_midi_note(octave: impl Into<i32>, note: impl Into<i32>) -> i32 {
    // Handle negative values and values beyond range
    let octave = wrap(octave.into(), 10);
    let note = wrap(note.into(), 12);
    // MIDI note number, where C4 is 60
    ((octave + 1) * 12) + note
}


#[inline(always)]
pub fn remap_range(value:f32, in_range:&RangeInclusive<f32>, out_range:&RangeInclusive<f32>) -> f32 {
    let source_range = in_range.end() - in_range.start();
    let x = (value - in_range.start()) / source_range;
    let dest_range = out_range.end() - out_range.start();
    (dest_range * x) + out_range.start()
}


#[inline(always)]
pub fn quantize_f32(value: f32, size: f32) -> f32 {
    roundf(value / size) * size
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
    // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
    let quantized_value = (roundf((value - min) / step_size) * step_size) + min;
    // Ensure the result is within the range after quantization
    quantized_value.clamp(min, max)
}

#[inline(always)]
pub fn quantize_f64(value: f64, size: f64) -> f64 {
    round(value / size) * size
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
    // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
    let quantized_value = (round((value - min) / step_size) * step_size) + min;
    // Ensure the result is within the range after quantization
    quantized_value.clamp(min, max)
}
