mod normal_signed;
pub use normal_signed::*;

mod normal;
pub use normal::*;

// A KnotValue is always converted to f32 for any math operation,
// so implementing all math ops seems unnecessary?
use core::{fmt, default};
pub trait KnotValue:
    Copy +
    Clone +
    Sized +
    PartialEq +
    From<f32> +
    Into<f32> +
    fmt::Debug +
    fmt::Display +
    default::Default
{
}

impl KnotValue for f32 {

}
