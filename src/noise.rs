
/// Noise profile.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Noise {
    #[default]
    /// No Noise
    None,
    /// Noise samples are completely random. WIP (Can't be pitched at all currently)
    Random { lfsr_length:u16, volume_steps:u16, noise_frequency:f32 },
    /// Noise samples are determined by an LFSR (Linear Feedback Shift Register) and
    /// can noticeably be pitched up or down.
    Melodic { lfsr_length:u16, volume_steps:u16, pitch_multiplier:f32 },
    /// A hybrid approach, each wavetable sample is mixed with noise resulting
    /// in a different wave on each cycle. Not implemented yet.
    WaveTable { mix: f32 }
}
