use crate::{
    eval::{State, Tester},
    parsing::{Parser, TokenLiterals, Tokens},
};

impl<'t> Tester<Tokens<'t>> {
    pub fn parse<'i: 't>(inp: &'i str) -> Self {
        let expr = Parser::parse(inp).shunting_yard();
        Self {
            var_count: expr.var_count(),
            expr,
        }
    }

    pub fn parse_with_literals<'i: 't>(inp: &'i str, literals: TokenLiterals) -> Self {
        let expr = Parser::parse_with_literals(inp, literals).shunting_yard();
        Self {
            var_count: expr.var_count(),
            expr,
        }
    }

    pub fn with_tokens(expr: Tokens<'t>) -> Self {
        Self {
            var_count: expr.var_count(),
            expr,
        }
    }

    pub fn var_at<S: State>(&self, s: S, n: usize) -> (&str, bool) {
        (self.expr.var_at(n), s.var_at(n))
    }

    pub fn vars(&self) -> &[&str] {
        self.expr.vars()
    }
}
