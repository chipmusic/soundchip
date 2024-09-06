/// Virtual sound chip's audio properties, which can be manipulated to mimic various
/// kinds of audio hardware per channel.
pub struct ChipSpecs {
    /// The output mix rate in Hertz, usually 44100 but depends on your sound playback device.
    pub sample_rate: u32,
    /// The number of steps per sample. For a PSG with only square waves this number doesn't matter
    /// as long as it's 1 or higher. For a simple wavetable like the SCC this number is 256 (1 byte).
    pub sample_steps: u16,
    /// The maximum number of volume states the chip can render, i.e. 4 bit volume register = 16 steps.
    pub volume_steps: u16,
    /// Quantizes the stereo pan state, i.e. 4 bit pan register = 16 steps.
    pub pan_steps: u16,
    /// Number of intermediate steps between note pitches.
    pub pitch_steps: u16,
    /// Values above 0.0 cause the signal to go drift back to zero after channel is playing. Higher values
    /// will more quickly reset to zero.
    pub volume_attenuation: f32,
    /// Adusts the volume envelope to a non-linear response.
    pub volume_exponent: f32,
    /// Allows certain chips to sound quieter or louder without affecting the channel's volume setting.
    pub volume_gain: f32,
    /// Certain chips (like the AY-3-8910) appear to only output positive values.
    pub prevent_negative_values:bool,
    /// Wether or not this channel can produce noise. 1980's chips usually had a single channel
    /// capable of generating noise.
    pub allow_noise: bool,
}

impl Default for ChipSpecs {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            sample_steps: 16,
            volume_steps: 16,
            pan_steps: 16,
            pitch_steps: 32,
            volume_attenuation: 0.0017,
            volume_exponent: 2.5,
            volume_gain: 2.5,
            prevent_negative_values: false,
            allow_noise: true,
        }
    }
}
