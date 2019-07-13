//! Boolean Expression Tokens

/// All the possible supported tokens in a
/// Boolean Expression
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<'a> {
    // Values
    Literal(bool),
    Var(&'a str, usize),

    // Operations
    Not,
    And,
    Xor,
    Or,
    Implication,
    Equality,

    // Parenthesis
    LParen,
    RParen,
    EOF(usize),
}

impl<'a> Token<'a> {
    pub fn precedence(&self) -> isize {
        match self {
            Self::Not => 5,
            Self::And => 4,
            Self::Xor => 3,
            Self::Or => 2,
            Self::Implication => 1,
            Self::Equality => 0,
            _ => isize::max_value(),
        }
    }
}

/// A list of all possible descriptions
/// for what each token could look like
/// in an input string
#[derive(Debug)]
pub struct TokenLiterals<'a> {
    pub literal_true: &'a [&'a str],
    pub literal_false: &'a [&'a str],
    pub not: &'a [&'a str],
    pub and: &'a [&'a str],
    pub xor: &'a [&'a str],
    pub or: &'a [&'a str],
    pub implication: &'a [&'a str],
    pub equality: &'a [&'a str],
    pub left_paren: &'a [&'a str],
    pub right_paren: &'a [&'a str],
}

impl Default for TokenLiterals<'static> {
    fn default() -> Self {
        Self {
            literal_true: &["true"],
            literal_false: &["false"],
            not: &["¬", "not", "!", "~"],
            and: &["∧", "and", "&&", "&", "*"],
            xor: &["⊕", "xor", "^"],
            or: &["∨", "or", "||", "|", "+"],
            implication: &["→", "->", "=>"],
            equality: &["≡", "<=>", "==", "="],
            left_paren: &["(", "{", "["],
            right_paren: &[")", "}", "]"],
        }
    }
}
