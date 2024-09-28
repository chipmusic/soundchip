// // Volume envelopes

// use core::cell::UnsafeCell;
// use core::ops::Deref;

// use crate::{prelude::*, presets::*};
// // use std::borrow::Borrow;

// // A simple spinlock-like mutex for no_std environments
// pub struct Mutex<T> {
//     data: UnsafeCell<T>,
// }


// static mut ENV_VOL_SAWTOOTH: Option<Envelope<Normal>>= None;

// pub fn env_vol_sawtooth() -> Option<&'static Envelope<Normal>> {
//     if let Some(env) = ENV_VOL_SAWTOOTH{
//         Some(&env)
//     } else {
//         let result =  Envelope::from(KNOTS_VOL_SAWTOOTH);
//         unsafe {
//             ENV_VOL_SAWTOOTH = Some(result);
//         }
//         Some(&result)
//     }
// }
