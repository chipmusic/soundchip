#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]
#![no_std]

extern crate alloc;
pub(crate) use alloc::vec::Vec;

pub mod math;

pub mod presets;

pub mod prelude;

pub mod rng;
