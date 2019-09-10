mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::{
    parsing::{Parser, Token, TokenLiterals, Tokens},
    tester::{State, Tester},
};

/// [`Tester`] based on parsed [`Tokens`].
///
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// parse the given input into a [`Tester`]
    ///
    /// [`Tester`]: `Tester`
    pub fn parse<'i: 't>(inp: &'i str) -> Self {
        let expr = Parser::parse(inp).shunting_yard();
        Self {
            state: State::default(expr.var_count()),
            expr,
        }
    }

    /// parse the given input into a [`Tester`] using the given [`TokenLiterals`]
    ///
    /// [`Tester`]: `Tester`
    /// [`TokenLiterals`]: `TokenLiterals`
    pub fn parse_with_literals<'i: 't>(inp: &'i str, literals: TokenLiterals) -> Self {
        let expr = Parser::parse_with_literals(inp, literals).shunting_yard();
        Self {
            state: State::default(expr.var_count()),
            expr,
        }
    }

    /// Create a new [`Tester`] from the given [`Tokens`]
    ///
    /// [`Tester`]: `Tester`
    /// [`Tokens`]: `Tokens`
    pub fn with_tokens(expr: Tokens<'t>) -> Self {
        Self {
            state: State::default(expr.var_count()),
            expr,
        }
    }

    /// Return the name and value of the variable at `idx`
    /// in the given [`State`].
    ///
    /// [`State`]: `State`
    pub fn var_at(&self, s: &State, idx: usize) -> (&str, bool) {
        (self.expr.var_at(idx), s.var_at(idx))
    }

    /// Returns all the variable names
    pub fn vars(&self) -> &[&str] {
        self.expr.vars()
    }

    pub(crate) fn eval_iter(&self, iter: usize) -> (State, bool) {
        use alloc::vec::Vec;
        use Token::*;

        let state = self.state.iterate(iter);
        let mut stack: Vec<bool> = Vec::new();
        // turn the expression into a single value
        for t in self.expr.tokens() {
            match *t {
                // Values can go directly to the stack
                Var(_, v) => stack.push(state.var_at(v)),
                Literal(v) => stack.push(v),
                // Operations
                Not => {
                    if let Some(v) = stack.pop() {
                        stack.push(!v);
                    }
                }
                And => {
                    if let Some(b) = stack.pop() {
                        if let Some(a) = stack.pop() {
                            stack.push(a && b);
                        }
                    }
                }
                Xor => {
                    if let Some(b) = stack.pop() {
                        if let Some(a) = stack.pop() {
                            stack.push(a ^ b);
                        }
                    }
                }
                Or => {
                    if let Some(b) = stack.pop() {
                        if let Some(a) = stack.pop() {
                            stack.push(a || b);
                        }
                    }
                }
                Implication => {
                    if let Some(b) = stack.pop() {
                        if let Some(a) = stack.pop() {
                            stack.push(!a || b);
                        }
                    }
                }
                Equality => {
                    if let Some(b) = stack.pop() {
                        if let Some(a) = stack.pop() {
                            stack.push(a == b);
                        }
                    }
                }
                LParen => unreachable!(),
                RParen => unreachable!(),
            }
        }

        if stack.len() != 1 {
            panic!("The expression did not resove to a single value!");
        }

        if let Some(v) = stack.pop() {
            (state, v)
        } else {
            unreachable!()
        }
    }
}
