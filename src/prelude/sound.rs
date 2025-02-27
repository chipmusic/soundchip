use crate::presets::KNOTS_VOL_DOWN;

use super::{Envelope, Normal, NormalSigned, Tremolo, Vibratto};

/// A single struct containing all public properties a sound can have,
/// such as volume, pitch, envelopes, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    /// Base volume excluding envelopes.
    pub volume: f32,
    /// Base pitch excluding envelopes.
    pub pitch: f32,
    /// Transitions from tone (0.0) to noise (1.0), if channel specs allow noise.
    pub noise_env: Option<Envelope<Normal>>,
    /// The channel's waveform. Some channel specs will set this and shouldn't be overriden.
    /// TODO: Stricter channel waveform if specs require it.
    pub waveform: Option<Envelope<NormalSigned>>,
    /// Optional volume tremolo. Acts as a secondary envelope subtracted from the regular volume envelope.
    pub tremolo: Option<Tremolo>,
    /// Optional pitch vibratto. Acts as a secondary envelope, added to the regular pitch envelope.
    pub vibratto: Option<Vibratto>,
    /// Optional volume envelope, range is 0.0 ..= 1.0
    pub volume_env: Option<Envelope<Normal>>,
    /// Optional pitch envelope. Range -1.0 ..= 1.0 means one octave down or up,
    /// but values can be beyond that range (use "envelope.scale_values(factor)"" to easily change that).
    pub pitch_env: Option<Envelope<f32>>,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pitch: 60.0,
            waveform: None,
            tremolo: None,
            vibratto: None,
            noise_env: None,
            volume_env: Some(Envelope::from(KNOTS_VOL_DOWN)),
            pitch_env: None,
        }
    }
}
