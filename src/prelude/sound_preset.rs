use super::*;

/// Allows a sound to be defined as const, but initialized at runtime since it needs
/// to call non-const functions. Member fields are the same as the ones in [Sound],
/// but with [EnvelopePreset] instead of [Envelope].
pub struct SoundPreset {
    pub volume: f32,
    pub pitch: f32,
    pub tremolo: Option<Tremolo>,
    pub vibratto: Option<Vibratto>,
    pub noise_env: Option<EnvelopePreset<Normal>>,
    pub waveform: Option<EnvelopePreset<NormalSigned>>,
    pub volume_env: Option<EnvelopePreset<Normal>>,
    pub pitch_env: Option<EnvelopePreset<f32>>,
}

impl From<SoundPreset> for Sound {
    fn from(preset: SoundPreset) -> Self {
        Self {
            volume: preset.volume,
            pitch: preset.pitch,
            tremolo: preset.tremolo,
            vibratto: preset.vibratto,
            noise_env: preset.noise_env.map(|p|{
                Envelope::from(p)
            }),
            waveform: preset.waveform.map(|p|{
                Envelope::from(p)
            }),
            volume_env: preset.volume_env.map(|p|{
                Envelope::from(p)
            }),
            pitch_env: preset.pitch_env.map(|p|{
                Envelope::from(p)
            }),
        }
    }
}
