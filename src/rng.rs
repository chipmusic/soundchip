//! A simple, "old school" LFSR with configurable bit count.
// use crate::{math::*, Vec};

#[derive(Debug)]
pub struct Rng {
    // bit_count: u32,
    state: u32,
    mask: u32,
    tap: u32,
    f32_max: f32,
}

const DEFAULT_VALUE: u32 = 0b01100010101011110110101010101011;

impl Rng {
    /// Creates a new LFSR with `n` bits and an initial state.
    pub fn new(bit_count: u32, initial_state: u32) -> Self {
        let bit_count = bit_count.clamp(3, 32);
        let mask = ((1u64 << bit_count) - 1) as u32;
        let state = if (initial_state & mask) == 0 {
            DEFAULT_VALUE & mask
        } else {
            initial_state & mask
        };
        Self {
            state,
            // bit_count,
            mask,
            tap: get_tap(bit_count),
            f32_max: libm::powf(2.0, bit_count as f32 ),
        }
    }

    // /// Returns a vec containing all values in a sequence (-1.0 to 1.0 range).
    // /// TODO: Customize range.
    // pub fn as_vec(bit_count:u32, initial_state:u32, volume_steps:u16) -> Vec<f32> {
    //     let mut rng = Self::new(bit_count, initial_state);
    //     let max = libm::powf(2.0, bit_count as f32) as usize - 1;
    //     (0..max).map(|_|{
    //         quantize_range(rng.next_f32(), volume_steps, -1.0 ..= 1.0)
    //     }).collect()
    // }

    /// Next random u32 value in the sequence
    pub fn next_u32(&mut self) -> u32 {
        let lsb = self.state & 1; // Store least significant bit
        self.state = xor_with_tap(self.state >> 1, self.tap, lsb);
        self.state & self.mask
    }

    /// Converts next random u32 value to range (0.0 .. 1.0)
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / self.f32_max
    }

    // /// Returns the value of the next least significant bit in the sequence.
    // pub fn next_bit(&mut self) -> bool {
    //     self.next_u32() & 1 == 1
    // }

    // /// How many bits are used to calculate the result
    // pub fn bit_count(&self) -> u32 {
    //     self.bit_count
    // }

    // /// Resets bit length
    // pub fn set_bit_count(&mut self, bit_count: u32, initial_state: u32) {
    //     // Update the bit length
    //     let bit_count = bit_count.clamp(3, 32);
    //     self.mask = (1u32 << bit_count) - 1;
    //     // self.bit_count = bit_count;
    //     self.tap = get_tap(bit_count);
    //     self.f32_max = libm::powf(2.0, bit_count as f32);
    //     // Apply the new initial state
    //     self.state = if (initial_state & self.mask) == 0 {
    //         DEFAULT_VALUE & self.mask
    //     } else {
    //         initial_state & self.mask
    //     };
    // }
}

#[inline(always)]
//  Bit can only be 0 or 1, or will overflow!
fn xor_with_tap(value: u32, tap: u32, bit: u32) -> u32 {
    value ^ (tap & (bit * 0xFFFFFFFF))
}

fn get_tap(bit_count: u32) -> u32 {
    // Maximal-length tap configurations.
    match bit_count {
        3 => 0b110,
        4 => 0b1100,
        5 => 0b10100,
        6 => 0b110000,
        7 => 0b1100000,
        8 => 0b10111000,
        9 => 0b100010000,
        10 => 0b1001000000,
        11 => 0b10100000000,
        12 => 0b111000001000,
        13 => 0b1110010000000,
        14 => 0b11100000000010,
        15 => 0b110000000000000,
        16 => 0b1101000000001000,
        17 => 0b10010000000000000,
        18 => 0b100000010000000000,
        19 => 0b1110010000000000000,
        20 => 0b10010000000000000000,
        21 => 0b101000000000000000000,
        22 => 0b1100000000000000000000,
        23 => 0b10000100000000000000000,
        24 => 0b111000010000000000000000, // Hand-entered up to here. The rest seems to work OK though...
        25 => 0b1000100000000000000000000,
        26 => 0b10010000000000000000000000,
        27 => 0b101000000000000000000000000,
        28 => 0b1011100000000000000000000000,
        29 => 0b11000000000000000000000000000,
        30 => 0b110010000000000000000000000000,
        31 => 0b1101000000000000000000000000000,
        32 => 0b11100000000000000000000000000000,
        _ => 0b101101,
    }
}
