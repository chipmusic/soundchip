/// Virtual sound chip's audio properties, which can be manipulated to mimic various
/// kinds of audio hardware per channel.
pub struct ChipSpecs {
    /// The output mix rate in Hertz, usually 44100 but depends on your sound playback device.
    pub sample_rate: u32,
    /// The maximum number of volume states the chip can render, i.e. 4 bit volume register = 16 steps.
    pub volume_steps: u16,
    /// Quantizes the stereo pan state, i.e. 4 bit pan register = 16 steps.
    pub pan_steps: u16,
    /// The number of steps per sample. For a PSG with only square waves this number doesn't matter
    /// as long as it's 1 or higher. For a simple wavetable like the SCC this number is 256 (1 byte).
    pub sample_steps: u16,
    /// Number of intermediate steps between note pitches.
    pub pitch_steps: u16,
    /// Wether or not this channel can produce noise. 1980's chips usually had a single channel
    /// capable of generating noise.
    pub allow_noise: bool,
    /// Values above 0.0 cause the signal to go drift back to zero after channel is playing. Higher values
    /// will more quickly reset to zero.
    pub attenuation: f32,
}

impl Default for ChipSpecs {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            volume_steps: 16,
            pan_steps: 16,
            sample_steps: 16,
            pitch_steps: 32,
            allow_noise: true,
            attenuation: 0.002,
        }
    }
}
