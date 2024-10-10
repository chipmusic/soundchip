use super::Normal;

/// Defines the envelope's looping behavior, if any.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoopKind {
    #[default]
    /// No loop. The last knot's value will be maintained.
    None,
    /// Repeats entire envelope, from the beginning.
    Repeat,
    /// Allows "sustaining" the envelope while it's not released. The sustain happens
    /// by looping between loop points, which are knot indices.
    LoopPoints{ loop_in:u8, loop_out:u8 },
    /// Similar to LoopPoints, but repeats entire envelope as an "echo" after its release,
    /// multiplying the amplitude by "decay" on each cycle.
    Echo{ loop_in:u8, loop_out:u8, decay:Normal },
}
