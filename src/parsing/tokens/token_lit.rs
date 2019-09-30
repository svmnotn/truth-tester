use super::Token;
use alloc::{collections::btree_map::BTreeMap, string::String, vec, vec::Vec};

#[derive(Debug, PartialEq)]
pub struct TokenLiterals {
    tokens: BTreeMap<Token<'static>, Vec<String>>,
}

impl TokenLiterals {
    pub fn lit_true(&self) -> &[String] {
        self.tokens
            .get(&Token::Literal(true))
            .expect("TokenLiteras was not crated properly, missing Literal(true)")
    }

    pub fn set_lit_true(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Literal(true), values);
    }

    pub fn lit_false(&self) -> &[String] {
        self.tokens
            .get(&Token::Literal(false))
            .expect("TokenLiteras was not crated properly, missing Literal(false)")
    }

    pub fn set_lit_false(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Literal(false), values);
    }

    pub fn not(&self) -> &[String] {
        self.tokens
            .get(&Token::Not)
            .expect("TokenLiteras was not crated properly, missing Not")
    }

    pub fn set_not(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Not, values);
    }

    pub fn and(&self) -> &[String] {
        self.tokens
            .get(&Token::And)
            .expect("TokenLiteras was not crated properly, missing And")
    }

    pub fn set_and(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::And, values);
    }

    pub fn xor(&self) -> &[String] {
        self.tokens
            .get(&Token::Xor)
            .expect("TokenLiteras was not crated properly, missing Xor")
    }

    pub fn set_xor(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Xor, values);
    }

    pub fn or(&self) -> &[String] {
        self.tokens
            .get(&Token::Or)
            .expect("TokenLiteras was not crated properly, missing Or")
    }

    pub fn set_or(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Or, values);
    }

    pub fn implication(&self) -> &[String] {
        self.tokens
            .get(&Token::Implication)
            .expect("TokenLiteras was not crated properly, missing Implication")
    }

    pub fn set_implication(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Implication, values);
    }

    pub fn equality(&self) -> &[String] {
        self.tokens
            .get(&Token::Equality)
            .expect("TokenLiteras was not crated properly, missing Equality")
    }

    pub fn set_equality(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::Equality, values);
    }

    pub fn left_paren(&self) -> &[String] {
        self.tokens
            .get(&Token::LParen)
            .expect("TokenLiteras was not crated properly, missing LParen")
    }

    pub fn set_left_paren(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::LParen, values);
    }

    pub fn right_paren(&self) -> &[String] {
        self.tokens
            .get(&Token::RParen)
            .expect("TokenLiteras was not crated properly, missing RParen")
    }

    pub fn set_right_paren(&mut self, values: Vec<String>) {
        self.tokens.insert(Token::RParen, values);
    }

    /// Does the given input string begin with one of our tokens, and if
    /// so which, and what is it's length.
    #[inline]
    pub fn starts_with<'a>(&self, input: &'a str) -> Option<(Token<'a>, usize)> {
        #[inline]
        fn find(input: &str, tokens: &[String]) -> Option<usize> {
            tokens
                .iter()
                .find(|t| {
                    if input.len() < t.len() {
                        false
                    } else {
                        t.eq_ignore_ascii_case(&input[..t.len()])
                    }
                })
                .map(|v| v.len())
        }

        for (t, v) in &self.tokens {
            if let Some(len) = find(input, v) {
                return Some((*t, len));
            }
        }

        None
    }
}

impl Default for TokenLiterals {
    fn default() -> Self {
        use Token::*;

        let mut map = BTreeMap::new();
        map.insert(
            Literal(true),
            vec!["true"].into_iter().map(Into::into).collect(),
        );
        map.insert(
            Literal(false),
            vec!["false"].into_iter().map(Into::into).collect(),
        );
        map.insert(
            Not,
            vec!["¬", "not", "!", "~"]
                .into_iter()
                .map(Into::into)
                .collect(),
        );
        map.insert(
            And,
            vec!["∧", "and", "&&", "&", "*"]
                .into_iter()
                .map(Into::into)
                .collect(),
        );
        map.insert(
            Xor,
            vec!["⊕", "xor", "^"].into_iter().map(Into::into).collect(),
        );
        map.insert(
            Or,
            vec!["∨", "or", "||", "|", "+"]
                .into_iter()
                .map(Into::into)
                .collect(),
        );
        map.insert(
            Implication,
            vec!["→", "->", "=>"].into_iter().map(Into::into).collect(),
        );
        map.insert(
            Equality,
            vec!["≡", "<=>", "==", "="]
                .into_iter()
                .map(Into::into)
                .collect(),
        );
        map.insert(
            LParen,
            vec!["(", "{", "["].into_iter().map(Into::into).collect(),
        );
        map.insert(
            RParen,
            vec![")", "}", "]"].into_iter().map(Into::into).collect(),
        );

        Self { tokens: map }
    }
}
