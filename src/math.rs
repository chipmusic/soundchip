
/// Provides quantization in a value with range -1.0 to 1.0.
#[inline(always)]
pub fn quantize(value:f32, size:f32) -> f32 {
    libm::roundf(value / size) * size
}

pub fn quantize_steps(value: f32, steps: u16) -> f32 {
    if steps == 0 { return 0.0 }
    if steps < 3 {
        return if value > 0.0 { 1.0 } else { -1.0 }
    }
    // Quantize
    let size = 1.0 / (steps - 1) as f32;
    quantize(value, size)
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Quantize {
//     pub min: f32,
//     pub max: f32,
//     pub steps: u16,
// }
