/// The Note enum can be used in lieu of MIDI note codes in any function that takes i32 as an argument.

#[repr(i32)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B
}

impl Into<i32> for Note {
    fn into(self) -> i32 {
        self as i32
    }
}
