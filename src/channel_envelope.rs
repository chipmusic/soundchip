// I'm calling it this instead of just "Envelope" to avoid a common conflict with other libraries.

/// Not implemented yet.
/// The behavior applied to the envelope once the timer reaches its end
pub enum LoopKind {
    /// Current value is sustained.
    Sustain,
    /// Value resets back to initial value and is sustained.
    Reset,
    /// Entire envelope resets back to inital value and proceeds normally, looping.
    Repeat,
    /// Envelope progresses back to previous value on each cycle.
    Mirror,
}

/// Not implemented yet. (May never be! Not sure how necessary this is, compared to simply
/// setting the volume on every frame, which effectively creates a volume envelope).
/// A Secondary volume envelope applied to a channel. Uses the same timing as the channel itself,
/// i.e. Channel::stop() will reset its cycle.
pub struct ChannelEnvelope {
    pub value_start: f32,
    pub value_end: f32,
    pub frequency: f32,
    pub kind: LoopKind,
}
