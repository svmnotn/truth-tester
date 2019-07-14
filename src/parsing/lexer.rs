use super::{Token, TokenLiterals};
use alloc::collections::btree_map::BTreeMap;
use core::{iter::Peekable, str::SplitWhitespace};

/// Boolean Expression Lexer
#[derive(Debug)]
pub struct Lexer<'t, 'l, 'i: 't> {
    literals: TokenLiterals<'l>,
    input: Peekable<SplitWhitespace<'i>>,
    curr_str: &'t str,
    var_map: BTreeMap<&'t str, usize>,
    finished: bool,
}

impl<'t, 'i: 't> Lexer<'t, 'static, 'i> {
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
            finished: false,
        }
    }
}

impl<'t, 'l, 'i: 't> Lexer<'t, 'l, 'i> {
    /// Create a Lexer with the given [`TokenLiterals`]
    ///
    /// [`TokenLiterals`]: `TokenLiterals`
    pub fn lex_with_literals(input: &'i str, literals: TokenLiterals<'l>) -> Self {
        let mut input = input.split_whitespace().peekable();
        let curr_str = input.next().unwrap_or("");
        Self {
            literals,
            input,
            curr_str,
            var_map: BTreeMap::new(),
            finished: false,
        }
    }
}

impl<'t, 'l, 'i: 't> Iterator for Lexer<'t, 'l, 'i> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        #[inline]
        fn find<'i, 'l>(inp: &'i str, tokens: &'l [&'l str]) -> Option<&'l &'l str> {
            tokens.iter().find(|token| {
                if inp.len() < token.len() {
                    false
                } else {
                    token.eq_ignore_ascii_case(&inp[0..token.len()])
                }
            })
        }

        if self.finished {
            None
        } else if self.input.peek().is_none() && self.curr_str.is_empty() {
            self.finished = true;
            Some(EOF(self.var_map.len()))
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
                if let Some(s) = find(&self.curr_str[i..], self.literals.lit_false) {
                    found_idx = i;
                    found_val = Some(Literal(false));
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.lit_true) {
                    found_idx = i;
                    found_val = Some(Literal(true));
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.not) {
                    found_idx = i;
                    found_val = Some(Not);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.and) {
                    found_idx = i;
                    found_val = Some(And);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.xor) {
                    found_idx = i;
                    found_val = Some(Xor);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.or) {
                    found_idx = i;
                    found_val = Some(Or);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.implication) {
                    found_idx = i;
                    found_val = Some(Implication);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.equality) {
                    found_idx = i;
                    found_val = Some(Equality);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.left_paren) {
                    found_idx = i;
                    found_val = Some(LParen);
                    found_val_len = s.len();
                    break;
                } else if let Some(s) = find(&self.curr_str[i..], self.literals.right_paren) {
                    found_idx = i;
                    found_val = Some(RParen);
                    found_val_len = s.len();
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
