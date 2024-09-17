/// Controls the wavetable length, quantization and looping. SoundChip always uses a wavetable, even
/// for square waves.
#[derive(Debug, Clone, PartialEq)]
pub struct WavetableSpecs {
    /// The length of the wavetable (how many samples per cycle)
    pub sample_count: usize,
    /// TODO: This will need a LoopKind enum.
    pub use_loop: bool,
    /// The number of steps per sample. For a PSG with only square waves this number doesn't matter
    /// as long as it's 1 or higher. For a simple wavetable like the SCC this number is 256 (1 byte).
    pub steps: Option<u16>,
}

impl Default for WavetableSpecs {
    fn default() -> Self {
        Self {
            steps: Some(16),
            sample_count: 16,
            use_loop: true,
        }
    }
}

impl WavetableSpecs {
    pub fn psg() -> Self {
        Self {
            steps: Some(2),
            sample_count: 8,
            use_loop: true,
        }
    }

    pub fn scc() -> Self {
        Self {
            steps: Some(256),
            sample_count: 32,
            use_loop: true,
        }
    }
}
