#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoopKind {
    #[default]
    None,
    Repeat,
    LoopPoints{ loop_in:u8, loop_out:u8 },
}
