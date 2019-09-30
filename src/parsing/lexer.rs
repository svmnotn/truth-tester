use super::{Token, TokenLiterals};
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use core::{iter::Peekable, str::SplitWhitespace};

/// Boolean Expression Lexer
#[derive(Debug)]
pub struct Lexer<'i> {
    literals: TokenLiterals,
    input: Peekable<SplitWhitespace<'i>>,
    curr_str: &'i str,
    var_map: BTreeMap<&'i str, usize>,
}

impl<'i> Lexer<'i> {
    /// Create a Lexer using the Default
    /// [`TokenLiterals`].
    ///
    /// [`TokenLiterals`]: `TokenLiterals`
    pub fn lex(input: &'i str) -> Self {
        let mut input = input.split_whitespace().peekable();
        let curr_str = input.next().unwrap_or("");
        Self {
            input,
            curr_str,
            literals: TokenLiterals::default(),
            var_map: BTreeMap::new(),
        }
    }

    /// Create a Lexer with the given [`TokenLiterals`]
    ///
    /// [`TokenLiterals`]: `TokenLiterals`
    pub fn lex_with_literals(input: &'i str, literals: TokenLiterals) -> Self {
        let mut input = input.split_whitespace().peekable();
        let curr_str = input.next().unwrap_or("");
        Self {
            literals,
            input,
            curr_str,
            var_map: BTreeMap::new(),
        }
    }

    /// Returns a Vec of var_names indexed by the order
    /// in which they appear in the expression
    ///
    /// *This function should only be called after
    /// the Lexer has finished*
    pub(crate) fn var_map(&self) -> Vec<&'i str> {
        let mut v: Vec<(&'i str, usize)> = self.var_map.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).expect("Unable to compare"));
        v.iter().map(|(name, _)| *name).collect()
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if self.input.peek().is_none() && self.curr_str.is_empty() {
            None
        } else {
            if self.curr_str.is_empty() {
                self.curr_str = self
                    .input
                    .next()
                    .expect("curr_str was empty, and next was None, but iter didn't finsh?!?");
            }

            let mut found_idx = 0;
            let mut found_val = None;
            let mut found_val_len = 0;

            for i in 0..self.curr_str.len() {
                let search_str = &self.curr_str[i..];
                if let Some((val, len)) = self.literals.starts_with(search_str) {
                    found_idx = i;
                    found_val = Some(val);
                    found_val_len = len;
                    break;
                }
            }

            if found_idx != 0 {
                // we found something, but there is a variable before it
                // get the variable name and index
                let name = &self.curr_str[..found_idx];
                let idx = self.var_map.len();
                // update our string with the remaining values
                self.curr_str = &self.curr_str[found_idx..];
                // return the variable
                Some(Var(name, *self.var_map.entry(name).or_insert(idx)))
            } else if found_val.is_some() {
                // we found a value
                // update the current string
                self.curr_str = &self.curr_str[found_val_len..];
                // return the found value
                found_val
            } else {
                // we found nothing, so the whole string is a variable
                // get the variable name and index
                let name = self.curr_str;
                let idx = self.var_map.len();
                // set our string to empty as we've taken all of it
                self.curr_str = "";
                // return our variable
                Some(Var(name, *self.var_map.entry(name).or_insert(idx)))
            }
        }
    }
}
