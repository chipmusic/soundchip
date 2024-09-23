#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoopKind {
    #[default]
    None,
    Repeat, // TODO: Think about "Repeat" and "envelope.release". Should "release" stop repeating?
    LoopPoints{ loop_in:u8, loop_out:u8 },
}
