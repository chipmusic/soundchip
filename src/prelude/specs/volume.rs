/// The processing specs for volume values.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecsVolume {
    /// The maximum number of volume states the chip can render, i.e. 4 bit volume register = 16 steps.
    pub steps: Option<u16>,
    /// Values above 0.0 cause the signal to go drift back to zero after channel is playing. Higher values
    /// will more quickly reset to zero.
    pub attenuation: f32,
    /// Adusts the volume envelope to a non-linear response. 1.0 is linear.
    /// Hardware manuals indicate this can be very aggressive, like 5.2.
    /// The default is a more conservative 3.0.
    pub exponent: f32,
    /// Allows certain chips to sound quieter or louder without affecting the channel's volume setting.
    pub gain: f32,
    /// Certain chips (like the AY-3-8910) appear to only output positive values.
    pub clip_negative_values: bool,
}

impl Default for SpecsVolume {
    fn default() -> Self {
        Self {
            steps: Some(16),
            attenuation: 0.001,
            exponent: 2.5,
            gain: 1.0,
            clip_negative_values: false,
        }
    }
}
