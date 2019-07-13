//! Here be the Boolean Expression Tester
mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use super::{Parser, Token};
use crate::State;
use alloc::vec::Vec;

pub struct Tester<'t> {
    state: State,
    expr: Vec<Token<'t>>,
}

impl<'t> Tester<'t> {
    pub fn parse<'i: 't>(inp: &'i str) -> Tester<'t> {
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

    /// Create a new `Tester`
    pub fn new(mut expr: Vec<Token<'t>>) -> Self {
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
