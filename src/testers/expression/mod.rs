mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::{
    parsing::{Parser, TokenLiterals, Tokens},
    State, Tester,
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
    pub fn parse_with_literals<'l, 'i: 't>(inp: &'i str, literals: TokenLiterals<'l>) -> Self {
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
}
