#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]
#![no_std]

mod sound_chip;
pub use sound_chip::*;

mod channel;
pub use channel::*;

mod sample;
pub use sample::*;

mod note;
pub use note::*;

mod error;
pub use error::*;

mod specs;
pub use specs::*;

mod envelope;
pub use envelope::*;

pub mod rng;
pub(crate) use rng::*;

pub mod math;
pub(crate) use math::*;

extern crate alloc;
pub(crate) use alloc::vec::Vec;


#[cfg(test)]
mod tests {

}
