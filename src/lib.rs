//! TODO better documentation
#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod testers;
#[cfg(feature = "alloc")]
pub use testers::ExprTester;
pub use testers::FnTester;

pub mod state;
pub use state::State;
