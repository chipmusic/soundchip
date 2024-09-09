use crate::PitchSpecs;

/// Noise profile.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Noise {
    #[default]
    /// No Noise
    None,
    /// Samples are determined by an LFSR (Linear Feedback Shift Register) and
    /// can noticeably be pitched up or down.
    Melodic {
        lfsr_length: u16,
        volume_steps: u16,
        pitch: PitchSpecs
    },
    /// LFSR Samples running at a fixed, usually very high frequency, but new samples
    /// can be skipped (and the current sample sustained) to pitch the resulting noise down.
    Random {
        lfsr_length: u16,
        volume_steps: u16,
        noise_frequency: f32,
        pitch: PitchSpecs
    },
    /// Not implemented yet.
    /// Wavetable samples are mixed with noise resulting in a different wave on each cycle.
    WaveTable { mix: f32 },
}
