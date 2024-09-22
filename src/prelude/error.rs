use core::fmt;

/// To be expanded.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChipError {
    InvalidIf16Range,
    InvalidUf16Range,
    InvalidWavetable,
    InvalidEnvelope,
}

impl fmt::Display for ChipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChipError::InvalidIf16Range => {
                write!(f, "Invalid If16: value out of -1.0 to 1.0 range")
            },
            ChipError::InvalidUf16Range => {
                write!(f, "Invalid Uf16: value out of 0.0 to 1.0 range")
            },
            ChipError::InvalidWavetable => {
                write!(f, "Invalid Wavetable: sample out of -1.0 to 1.0 range")
            },
            ChipError::InvalidEnvelope => {
                write!(f, "Invalid Envelope: knot value out of -1.0 to 1.0 range")
            }
        }
    }
}
