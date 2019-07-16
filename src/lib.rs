//! TODO better documentation
#![no_std]
#![warn(missing_docs)]
// This is used in order to have `ExprFn` exist,
// can be removed once `trait_alias` stabilizes,
// see https://github.com/rust-lang/rust/issues/41517
#![feature(trait_alias)]

#[cfg(feature = "parsing")]
extern crate alloc;

#[cfg(feature = "tester")]
pub mod tester;

#[cfg(feature = "parsing")]
pub mod parsing;
