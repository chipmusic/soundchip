use crate::prelude::*;

/// Noise profile.  Controls whether this channel can produce noise.
/// The frequency multiplier in the "pitch" struct is applied *after* quantizing the pitch,
/// allowing you to map a typical MIDI range like C3 to C5 to a much higher frequency
#[derive(Debug, Clone, PartialEq)]
pub enum SpecsNoise {
    /// No Noise
    None,
    /// Samples are determined by an LFSR (Linear Feedback Shift Register) and
    /// can noticeably be pitched up or down.
    Melodic {
        lfsr_length: u16,
        volume_steps: u8,
        pitch: SpecsPitch,
    },
    /// LFSR Samples running at a fixed, usually very high frequency, but new samples
    /// can be skipped (and the current sample sustained) to pitch the resulting noise down.
    Random {
        volume_steps: u8,
        pitch: SpecsPitch,
    },
    /// Not implemented yet.
    /// Wavetable samples are mixed with noise resulting in a different wave on each cycle.
    WaveTable { mix: f32 },
}

impl Default for SpecsNoise {
    /// Returns a TIA-like, metalic sounding noise profile but with relaxed pitch restrictions.
    fn default() -> Self {
        Self::Melodic {
            lfsr_length: 5,
            volume_steps: 2,
            pitch: SpecsPitch {
                multiplier: 5.0,
                ..Default::default()
            },
        }
    }
}
