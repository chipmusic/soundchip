//! Everything you need to get a SoundChip up and running.
//! Start witht the SoundChip, Channel and Sound structs and go from there.

mod specs;
pub use specs::*;

mod values;
pub use values::*;

mod channel;
pub use channel::*;

mod envelope;
pub use envelope::*;

mod error;
pub use error::*;

mod loop_kind;
pub use loop_kind::*;

mod note;
pub use note::*;

mod sample;
pub use sample::*;

mod sound;
pub use sound::*;

mod sound_chip;
pub use sound_chip::*;
