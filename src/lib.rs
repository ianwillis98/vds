#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # vds
//! A `#![no_std]` crate for visibly distinguishable strings and codes.
//!
//! This crate provides:
//!
//! - [`VDChar`]: a compact, index-based character type
//! - [`VDString`]: a validated string of `VDChar`s
//! - [`VDGenerator`]: a builder for random string generation *(requires `generate` feature)*
//!
//! ## Features
//!
//! - `generate` — enables [`VDGenerator`] for random string creation (uses `rand_core`)
//! - `serde` — enables `Serialize` / `Deserialize` support via the `serde` crate

mod vdchar;
mod vdstring;
#[cfg(feature = "generate")]
mod generate;
#[cfg(feature = "serde")]
mod serde;

pub use vdchar::{VDChar, VDS_ALLOWED};
pub use vdstring::{VDString, VDStringError};

#[cfg(feature = "generate")]
pub use generate::{VDGenerator, VDGeneratorError};