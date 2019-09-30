#[cfg(feature = "tester")]
mod exper;

mod token;
pub use token::Token;

mod token_lit;
pub use token_lit::TokenLiterals;

use alloc::vec::Vec;

pub struct Tokens<'a> {
    toks: Vec<Token<'a>>,
    var_map: Vec<&'a str>,
    var_count: usize,
}

impl<'a> Tokens<'a> {
    pub(crate) fn new(toks: Vec<Token<'a>>, var_map: Vec<&'a str>) -> Self {
        Tokens {
            var_count: var_map.len(),
            var_map,
            toks,
        }
    }

    pub fn var_at(&self, n: usize) -> &str {
        self.var_map[n]
    }

    pub fn vars(&self) -> &[&str] {
        &self.var_map
    }

    pub fn var_count(&self) -> usize {
        self.var_count
    }
}
