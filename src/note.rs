/// The Note enum can be used in lieu of MIDI note codes in any function
/// that takes Into<i32>, Into<f32> or Into<f64> as an argument.

#[repr(u8)]
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

impl Into<f32> for Note {
    fn into(self) -> f32 {
        (self as u8) as f32
    }
}

impl Into<f64> for Note {
    fn into(self) -> f64 {
        (self as u8) as f64
    }
}
