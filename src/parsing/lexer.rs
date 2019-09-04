use super::{Token, TokenLiterals};
use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};
use core::{iter::Peekable, str::SplitWhitespace};

/// Boolean Expression Lexer
#[derive(Debug)]
pub struct Lexer<'t, 'i: 't> {
    literals: TokenLiterals,
    input: Peekable<SplitWhitespace<'i>>,
    curr_str: &'t str,
    var_map: BTreeMap<&'t str, usize>,
}

impl<'t, 'i: 't> Lexer<'t, 'i> {
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
}

impl<'t, 'i: 't> Lexer<'t, 'i> {
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

    /// Returns a Vec of var_names index by the order
    /// in which they appear in the expression
    ///
    /// *This function should only be called after
    /// the Lexer has finished*
    pub(crate) fn var_map(&self) -> Vec<&'t str> {
        self.var_map.iter().map(|(name, _)| *name).collect()
    }
}

impl<'t, 'i: 't> Iterator for Lexer<'t, 'i> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        #[inline]
        fn find<'i>(
            inp: &'i str,
            tokens: &[String],
            t: Token<'i>,
        ) -> Option<(Token<'i>, usize)> {
            tokens
                .iter()
                .find(|token| {
                    if inp.len() < token.len() {
                        false
                    } else {
                        token.eq_ignore_ascii_case(&inp[0..token.len()])
                    }
                })
                .map(|s| (t, s.len()))
        }

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
                if let Some((val, len)) = find(search_str, &self.literals.lit_true, Literal(true))
                    .or_else(|| find(search_str, &self.literals.lit_false, Literal(false)))
                    .or_else(|| find(search_str, &self.literals.not, Not))
                    .or_else(|| find(search_str, &self.literals.and, And))
                    .or_else(|| find(search_str, &self.literals.xor, Xor))
                    .or_else(|| find(search_str, &self.literals.or, Or))
                    .or_else(|| find(search_str, &self.literals.implication, Implication))
                    .or_else(|| find(search_str, &self.literals.equality, Equality))
                    .or_else(|| find(search_str, &self.literals.left_paren, LParen))
                    .or_else(|| find(search_str, &self.literals.right_paren, RParen))
                {
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
