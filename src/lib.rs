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
pub use testers::{Tester, ExprFn};
#[cfg(feature = "alloc")]
pub use testers::{Lexer, Parser, Token, Tokens, TokenLiterals};

mod state;
pub use state::State;
