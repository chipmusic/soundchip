use super::{SpecsPan, SpecsWavetable, SpecsVolume, SpecsNoise, SpecsPitch};

/// Virtual sound chip's audio properties, which can be manipulated to mimic various
/// kinds of audio hardware per channel.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpecsChip {
    pub wavetable:SpecsWavetable,
    pub pan: SpecsPan,
    pub pitch: SpecsPitch,
    pub volume: SpecsVolume,
    pub noise: SpecsNoise,
}
