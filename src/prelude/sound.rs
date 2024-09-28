use super::{Envelope, Normal, NormalSigned, Tremolo, Vibratto};

pub struct Sound {
    pub volume: f32,
    pub pitch: f32,
    pub waveform: Option<Envelope<NormalSigned>>,
    pub tremolo: Option<Tremolo>,
    pub vibratto: Option<Vibratto>,
    pub volume_envelope: Option<Envelope<Normal>>,
    pub pitch_envelope: Option<Envelope<f32>>,
}
