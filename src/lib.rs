//! The SoundChip struct contains multiple channels, each with configurable settings that can
//! replicate old audio chips like PSGs and simple wave table chips.

#![no_std]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]

mod channel;
pub use channel::*;

mod sample;
pub use sample::*;

mod note;
pub use note::*;

extern crate alloc;
use alloc::vec::Vec;

/// Contains multiple sound channels, and can render them all at once. The resulting buffer can
/// be iterated to obtain the final stereo samples (interleaved, left then right).
pub struct SoundChip {
    pub is_playing: bool,
    pub output_mix_rate: u32,
    channels: Vec<Channel>,
    // buffer: Vec<i16>,
    sample_head: usize,
    last_sample_time: f64,
}

const MAX_VOL: f32 = (i16::MAX - 100) as f32;

impl Default for SoundChip {
    fn default() -> Self {
        Self {
            is_playing: false,
            sample_head: 0,
            output_mix_rate: 44100,
            channels: (0..4)
                .map(|_| Channel::new(44100, 16, 16, 16, true))
                .collect(),
            last_sample_time: 0.0,
        }
    }
}

impl SoundChip {
    /// Creates a SoundChip pre-configured to generate 4 channels, each with a 16x16 wavetable.
    pub fn new(mix_rate: u32) -> Self {
        Self {
            channels: (0..4)
                .map(|_| Channel::new(mix_rate, 16, 16, 16, true))
                .collect(),
            ..Default::default()
        }
    }

    /// Creates a SoundChip configured to replicate an AY-3-8910 sound chip with 3 square wave channels.
    pub fn new_msx(mix_rate: u32) -> Self {
        Self {
            channels: (0..3)
                .map(|i| match i {
                    0 => Channel::new(mix_rate, 16, 16, 8, true),
                    _ => Channel::new(mix_rate, 16, 16, 8, false),
                })
                .collect(),
            ..Default::default()
        }
    }

    /// Creates a SoundChip configured to replicate an AY-3-8910 sound chip with 3 square wave channels plus
    /// an SCC chip with 5 wavetable channels (32 byte samples).
    pub fn new_msx_scc(mix_rate: u32) -> Self {
        Self {
            channels: (0..3)
                .map(|i| match i {
                    0 => Channel::new(mix_rate, 16, 16, 8, true),
                    1 | 2 => Channel::new(mix_rate, 16, 16, 8, false),
                    _ => Channel::new(mix_rate, 16, 256, 32, false),
                })
                .collect(),
            ..Default::default()
        }
    }

    /// Returns a reference to the channel with the given index, None if channel doesn't exist.
    pub fn channel(&mut self, index: usize) -> Option<&mut Channel> {
        self.channels.get_mut(index)
    }

    /// Iterates through every sample currently in the buffer.
    /// Use [SoundChip::process_samples] to fill the buffer.
    pub fn iter(&mut self, sample_count:usize) -> SoundChipIter {
        SoundChipIter {
            chip: self,
            head: 0,
            sample_count
        }
    }

    /// Process a single sample
    pub fn process_sample(&mut self) -> Sample<i16> {
        let mut left: f32 = 0.0;
        let mut right: f32 = 0.0;

        let time = self.sample_head as f64 / self.output_mix_rate as f64;
        let delta_time = time - self.last_sample_time;
        self.last_sample_time = time;

        for channel in &mut self.channels {
            let sample = channel.sample(delta_time);
            left += sample.left;
            right += sample.right;
        }

        let len = self.channels.len() as f32;
        let left = ((left / len).clamp(-1.0, 1.0) * MAX_VOL) as i16;
        let right = ((right / len).clamp(-1.0, 1.0) * MAX_VOL) as i16;
        self.sample_head += 1;
        Sample { left, right }
    }
}

/// Iterates all values currently in the buffer. Use [SoundChip::process_samples] to populate the buffer.
pub struct SoundChipIter<'a> {
    chip: &'a mut SoundChip,
    head: usize,
    sample_count:usize
}

impl<'a> Iterator for SoundChipIter<'a> {
    type Item = Sample<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < self.sample_count{
            self.head += 1;
            return Some(self.chip.process_sample());
        }
        None
    }
}
