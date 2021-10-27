/*! `wyz` – myrrlyn’s wyzyrdly library

This crate consolidates all the small tools and conveniences I’ve built up in my
experience building Rust crates.

Each module has more documentation about what it contains. The modules are
largely independent, and can be used individually.
!*/

#![no_std]
#![cfg_attr(debug_assertions, warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod bidi;
pub mod comu;
pub mod fmt;

#[cfg(all(feature = "std", feature = "garbage"))]
pub mod wm;

#[cfg(feature = "std")]
#[macro_use]
pub mod exit;

pub use comu::*;
pub use fmt::*;

#[cfg(feature = "std")]
pub use exit::*;

#[cfg(all(feature = "std", feature = "garbage"))]
pub use wm::{
	BgDrop,
	BgDropExt,
};
