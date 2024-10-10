use crate::prelude::{Knot, NormalSigned};
use crate::presets::*;

/// Controls the wavetable length, quantization and looping. SoundChip always uses a wavetable, even
/// for square waves.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecsWavetable {
    /// Optional default waveform, applied to the wavetable when creating a
    /// new channel using Channel::from(specs).
    pub default_waveform: Option<&'static [Knot<NormalSigned>]>,
    /// The length of the wavetable (how many samples per cycle)
    pub sample_count: usize,
    /// TODO: This will need a LoopKind enum.
    pub use_loop: bool,
    /// The number of steps per sample. For a PSG with only square waves this number doesn't matter
    /// as long as it's 1 or higher. For a simple wavetable like the SCC this number is 256 (1 byte).
    pub steps: Option<u16>,
}

impl Default for SpecsWavetable {
    fn default() -> Self {
        Self {
            default_waveform: Some(KNOTS_WAVE_TRIANGLE),
            steps: Some(32),
            sample_count: 32,
            use_loop: true,
        }
    }
}
