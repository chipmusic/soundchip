#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]
#![no_std]

extern crate alloc;
pub(crate) use alloc::vec::Vec;

pub mod math;

pub mod presets;

/// Everything you need to get a SoundChip up and running.
/// Start witht the SoundChip and Channel structs and go from there.
pub mod prelude;

pub(crate) mod rng;
