/// Vibratto specs, a secondary pitch envelope that "wobbles" the pitch up and down with a sine wave,
/// optionally quantized to the number of steps. Amplitude of 1.0 means a whole octave up and down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vibratto {
    pub steps:Option<u16>,
    pub amplitude:f32,
    pub frequency:f32,
}
