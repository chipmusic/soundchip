
/// Not implemented yet.
/// Noise profile.
pub enum NoiseKind {
    /// Noise samples are completely random.
    Random { volume_steps:u32, pitch_multiplier:f32 },
    /// Noise samples are determined by an LFSR (Linear Feedback Shift Register).
    PseudoRandom { lfsr_length:u16, volume_steps:u32, pitch_multiplier:f32 },
    /// A hybrid approach, each wavetable sample is mixed with noise resulting in a different wave on each cycle.
    WaveTable { mix: f32 }
}
