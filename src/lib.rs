#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "core")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

extern crate approx_cmp;
extern crate num_traits;


mod coordinates;

pub mod e2ga;
pub mod e3ga;
pub mod scalar;
