mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::{Tester, State, parsing::{Parser, Token, Tokens, TokenLiterals}};

/// [`Tester`] based on parsed [`Tokens`].
/// 
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// parse the given input into a [`Tester`]
    /// 
    /// [`Tester`]: `Tester`
    pub fn parse<'i: 't>(inp: &'i str) -> Self {
        let mut expr = Parser::parse(inp).shunting_yard();
        if let Token::EOF(var_count) = expr.pop().expect("Expected a non-empty expression") {
            Self {
                state: State::default(var_count),
                expr,
            }
        } else {
            panic!("Last token was not EOF");
        }
    }

    /// parse the given input into a [`Tester`] using the given [`TokenLiterals`]
    /// 
    /// [`Tester`]: `Tester`
    /// [`TokenLiterals`]: `TokenLiterals`
    pub fn parse_with_literals<'l, 'i: 't>(inp: &'i str, literals: TokenLiterals<'l>) -> Self {
        let mut expr = Parser::parse_with_literals(inp, literals).shunting_yard();
        if let Token::EOF(var_count) = expr.pop().expect("Expected a non-empty expression") {
            Self {
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
            Self {
                state: State::default(var_count),
                expr,
            }
        } else {
            panic!("Last token was not EOF");
        }
    }
}
