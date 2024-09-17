#[derive(Debug, Default, Clone)]
pub enum EnvelopeState {
    #[default]
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}
