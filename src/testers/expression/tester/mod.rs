mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::{Tester, State, Parser, Token, Tokens};

/// [`Tester`] based on parsed [`Tokens`].
/// 
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// parse the given input into a [`Tester`]
    /// 
    /// [`Tester`]: `Tester`
    pub fn parse<'i: 't>(inp: &'i str) -> Self {
        let mut expr = Parser::new(inp).shunting_yard();
        if let Token::EOF(var_count) = expr.pop().expect("Expected a non-empty expression") {
            Tester {
                state: State::default(var_count),
                expr,
            }
        } else {
            panic!("Last token was not EOF");
        }
    }

    /// Create a new [`Tester`] from the given [`Tokens`]
    /// 
    /// [`Tester`]: `Tester`
    /// [`Tokens`]: `Tokens`
    pub fn with_tokens(mut expr: Tokens<'t>) -> Self {
        if let Token::EOF(var_count) = expr.pop().expect("Expected a non-empty expression") {
            Tester {
                state: State::default(var_count),
                expr,
            }
        } else {
            panic!("Last token was not EOF");
        }
    }
}
