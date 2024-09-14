use super::{PanSpecs, WavetableSpecs, VolumeSpecs, NoiseSpecs, PitchSpecs};

/// Virtual sound chip's audio properties, which can be manipulated to mimic various
/// kinds of audio hardware per channel.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChipSpecs {
    pub wavetable:WavetableSpecs,
    pub pan: PanSpecs,
    pub pitch: PitchSpecs,
    pub volume: VolumeSpecs,
    pub noise: NoiseSpecs,
}
