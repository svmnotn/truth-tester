//! TODO better documentation
#![no_std]
#![feature(trait_alias)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod testers;
#[cfg(feature = "alloc")]
pub use testers::{ExprTester, Lexer, Parser, Token, TokenLiterals};
pub use testers::{FnTester, ExprFn};

mod state;
pub use state::State;
