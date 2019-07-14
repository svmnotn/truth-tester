//! TODO better documentation
#![no_std]
#![warn(missing_docs)]
// This is used in order to have `ExprFn` exist,
// can be removed once `trait_alias` stabilizes,
// see https://github.com/rust-lang/rust/issues/41517
#![feature(trait_alias)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod testers;
#[cfg(feature = "alloc")]
pub use testers::{ExprTester, Lexer, Parser, Token, TokenLiterals};
pub use testers::{FnTester, ExprFn};

mod state;
pub use state::State;
