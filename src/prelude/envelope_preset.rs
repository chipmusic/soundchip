use super::{Knot, KnotValue, LoopKind};

/// Allows Envelope presets to be defined as consts, and the resulting envelope
/// (which calls non-const functions) can be initialized at runtime.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopePreset<T>
where
    T: KnotValue + 'static + Clone,
{
    pub knots: &'static [Knot<T>],
    pub time_scale: f32,
    pub value_scale: f32,
    pub value_offset: f32,
    pub loop_kind: LoopKind,
}
