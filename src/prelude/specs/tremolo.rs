
/// Tremolo specs, a secondary volume envelope that "wobbles" the volume up and down with a sine wave,
/// optionally quantized to the number of steps.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecsTremolo {
    pub steps:Option<u16>,
    pub amplitude:f32,
    pub frequency:f32,
}
