use core::fmt;

/// To be expanded.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChipError {
    InvalidWavetable
}

impl fmt::Display for ChipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChipError::InvalidWavetable => write!(f, "Invalid Wavetable: sample out of -1.0 to 1.0 range"),
        }
    }
}
