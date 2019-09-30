#![no_std]
#![warn(missing_docs)]
// This is for conditional compilation of code examples
#![feature(external_doc)]

#[cfg(feature = "parsing")]
extern crate alloc;

#[cfg(feature = "tester")]
pub mod eval;

#[cfg(feature = "parsing")]
pub mod parsing;
