use crate::PitchSpecs;

/// Noise profile.  Controls wether or not this channel can produce noise.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseSpecs {
    /// No Noise
    None,
    /// Samples are determined by an LFSR (Linear Feedback Shift Register) and
    /// can noticeably be pitched up or down.
    Melodic {
        lfsr_length: u16,
        volume_steps: u16,
        pitch: PitchSpecs,
    },
    /// LFSR Samples running at a fixed, usually very high frequency, but new samples
    /// can be skipped (and the current sample sustained) to pitch the resulting noise down.
    Random {
        volume_steps: u16,
        pitch: PitchSpecs,
    },
    /// Not implemented yet.
    /// Wavetable samples are mixed with noise resulting in a different wave on each cycle.
    WaveTable { mix: f32 },
}

impl Default for NoiseSpecs {
    fn default() -> Self {
        Self::Melodic {
            lfsr_length: 5,
            volume_steps: 2,
            pitch: PitchSpecs {
                multiplier: 5.0,
                steps: None,
                range: None,
            },
        }
    }
}

impl NoiseSpecs {
    pub fn psg(allow_noise:bool) -> Self {
        if allow_noise {
            Self::Random {
                volume_steps: 1,
                pitch: PitchSpecs {
                    multiplier: 5.0,
                    steps: Some(32),
                    range: Some(130.81 .. 783.99),
                },
            }
        } else {
            Self::None
        }
    }
}
