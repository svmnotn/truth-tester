//! Here are all the Boolean Expression Parsing
//! data structures
//!
//! ### Lexing
//! To use `truth_tester` to lex a boolean expression, simply import
//! the [`parsing`] module's [`Lexer`], [`Token`], and
//! optinally the [`TokenLiterals`], if you wish to change the default
//! behavior of what is understood by the [`Lexer`].
//!
//! [`Lexer`]: `crate::parsing::Lexer`
//! [`Token`]: `crate::parsing::Token`
//! [`TokenLiterals`]: `crate::parsing::TokenLiterals`
mod lexer;
mod parser;
mod tokens;

pub use lexer::Lexer;
pub use parser::Parser;
pub use tokens::{Token, TokenLiterals, Tokens};
