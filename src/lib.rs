#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]
#![no_std]

extern crate alloc;
pub(crate) use alloc::vec::Vec;

pub mod math;

pub mod rng;

pub mod presets;

/// Everything you need to get a SoundChip up and running.
/// Start witht the SoundChip and Channel structs and go from there.
pub mod prelude {
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
}
