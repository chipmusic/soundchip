use crate::presets::KNOTS_VOL_SAWTOOTH;

use super::{Envelope, Normal, NormalSigned, Tremolo, Vibratto};

#[derive(Debug, Clone)]
pub struct Sound {
    pub volume: f32,
    pub pitch: f32,
    pub waveform: Option<Envelope<NormalSigned>>,
    pub tremolo: Option<Tremolo>,
    pub vibratto: Option<Vibratto>,
    pub volume_envelope: Option<Envelope<Normal>>,
    pub pitch_envelope: Option<Envelope<f32>>,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pitch: 60.0,
            waveform: None,
            tremolo: None,
            vibratto: None,
            volume_envelope: Some(Envelope::from(KNOTS_VOL_SAWTOOTH)),
            pitch_envelope: None,
        }
    }
}
