//! Helpful math functions that only depend on libm.

use core::ops::RangeInclusive;
use libm::{roundf, sinf};

/// Returns the MIDI note value given an octave (zero to 10) and a note (zero to 11).
#[inline(always)]
pub fn get_midi_note(octave: impl Into<i32>, note: impl Into<i32>) -> i32 {
    // Handle negative values and values beyond range
    let octave = wrap(octave.into(), 10);
    let note = wrap(note.into(), 12);
    // MIDI note number, where C4 is 60
    ((octave + 1) * 12) + note
}

#[inline(always)]
pub fn note_to_frequency(note: f32) -> f32 {
    libm::powf(2.0, (note - 69.0) / 12.0) * 440.0
}

#[inline(always)]
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

#[inline(always)]
pub fn wrap(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}

#[inline(always)]
pub(crate) fn compress_volume(input_vol:f32, max_vol:f32) -> f32 {
    let mult = core::f32::consts::FRAC_2_PI;
    sinf(input_vol/(max_vol*mult))
}

#[inline(always)]
pub fn remap_range(value:f32, in_range:&RangeInclusive<f32>, out_range:&RangeInclusive<f32>) -> f32 {
    let source_range = in_range.end() - in_range.start();
    let x = (value - in_range.start()) / source_range;
    let dest_range = out_range.end() - out_range.start();
    (dest_range * x) + out_range.start()
}

// #[inline(always)]
// pub(crate) fn quantize(value: f32, size: f32) -> f32 {
//     roundf(value / size) * size
// }

pub(crate) fn quantize_range(value: f32, steps: u16, range: RangeInclusive<f32>) -> f32 {
    // Fewer than two steps returns zero, useful in setting the pan
    if steps < 2 {
        return 0.0;
    }
    let steps = steps - 1;
    let min = *range.start();
    let max = *range.end();
    let step_size = (max - min) / steps as f32;
    // Find the nearest step by dividing the clamped value by step size, rounding it, and multiplying back
    let quantized_value = (roundf((value - min) / step_size) * step_size) + min;
    // Ensure the result is within the range after quantization
    quantized_value.clamp(min, max)
}


#[test]
fn quantization_test() {
    let mut last_value = 0.0;
    let mut value_count = 0;
    let steps = 5;
    for n in -10 ..= 10 {
        let value = n as f32 / 10.0;
        let result = quantize_range(value, steps, -1.0 ..= 1.0);
        if result != last_value {
            last_value = result;
            value_count += 1;
        }
        // println!("{:.3} => {:.3}", value, result);
    }
    assert_eq!(steps, value_count);
}


#[test]
fn remap_test(){
    let a = remap_range(1.0, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(a, 5.0);

    let b = remap_range(2.0, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(b, 10.0);

    let c = remap_range(1.5, &(1.0 ..= 2.0), &(5.0 ..= 10.0));
    assert_eq!(c, 7.5);

    let d = remap_range(0.0, &(-1.0 ..= 1.0), &(0.0 ..= 1.0));
    assert_eq!(d, 0.5);

    let d = remap_range(0.5, &(0.0 ..= 1.0), &(-1.0 ..= 1.0));
    assert_eq!(d, 0.0);

    let d = remap_range(0.0, &(0.0 ..= 1.0), &(-1.0 ..= 1.0));
    assert_eq!(d, -1.0);

    // Inverted range
    let e = remap_range(0.0, &(0.0 ..= 1.0), &(0.0 ..= -1.0));
    assert_eq!(e, 0.0);

    let f = remap_range(1.0, &(0.0 ..= 1.0), &(0.0 ..= -1.0));
    assert_eq!(f, -1.0);
}
