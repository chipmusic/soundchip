#[derive(Debug, Default, Clone)]
pub enum EnvelopeState {
    #[default]
    Attack,
    Decay,
    Sustain,
    Release,
    Idle
}
