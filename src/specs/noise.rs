use crate::{Note, PitchSpecs};

/// Noise profile.  Controls whether this channel can produce noise.
/// The frequency multiplier in the "pitch" struct is applied *after* quantizing the pitch,
/// allowing you to map a typical MIDI range like C3 to C5 to a much higher frequency
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
    /// Returns a TIA-like, metalic soundng noise profile.
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
    /// Returns a PSG-like noise profile with 32 valid pitch values between C3 and G#5.
    pub fn psg(allow_noise:bool) -> Self {
        let min = Note::C.frequency(3);
        let max = Note::GSharp.frequency(5);
        if allow_noise {
            Self::Random {
                volume_steps: 1,
                pitch: PitchSpecs {
                    multiplier: 55.0,
                    steps: Some(32),
                    range: Some(min ..= max),
                },
            }
        } else {
            Self::None
        }
    }
}
