use super::{SpecsPan, SpecsWavetable, SpecsVolume, SpecsNoise, SpecsPitch};

/// Sound chip's audio properties, which can be manipulated to mimic various
/// kinds of audio hardware per channel.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpecsChip {
    /// With the default value of Some(60.0) in Hertz, envelopes aren't processed on every sample.
    /// The last envelope value will be re-used during each period, which is very accurate to how
    /// many 80's and 90's games processed sound (i.e. once every video frame).
    /// Affects Volume and Pitch envelopes, as well as tremolo and vibratto.
    pub envelope_rate: Option<f32>,
    pub wavetable:SpecsWavetable,
    pub pan: SpecsPan,
    pub pitch: SpecsPitch,
    pub volume: SpecsVolume,
    pub noise: SpecsNoise,
}
