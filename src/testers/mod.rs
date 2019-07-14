mod func;
pub use func::{Tester as FnTester, ExprFn};

#[cfg(feature = "alloc")]
mod expression;
#[cfg(feature = "alloc")]
pub use expression::{ExprTester, Token, TokenLiterals, Tokens, Parser, Lexer};
