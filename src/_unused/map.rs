
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeMap {
    None,
    Scale(f32),
    CenteredScale(f32)
}
