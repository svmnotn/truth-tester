use alloc::{string::String, vec, vec::Vec};

/// A full expression, which is made of [`Token`]s
///
/// [`Token`]: `Token`
pub struct Tokens<'a> {
    pub(super) toks: Vec<Token<'a>>,
    pub(super) var_map: Vec<&'a str>,
    pub(super) var_count: usize,
}

impl<'a> Tokens<'a> {
    /// Return an iterator over all the [`Token`]s in the expression
    ///
    /// [`Token`]: `Token`
    pub fn tokens(&self) -> impl Iterator<Item = &Token<'a>> {
        self.toks.iter()
    }

    /// Return the name of the `idx` variable in the expression
    pub fn var_at(&self, idx: usize) -> &str {
        self.var_map[idx]
    }

    /// A list of all the variable names
    /// ordered by occurence in the
    /// expression
    pub fn vars(&self) -> &[&str] {
        &self.var_map
    }

    /// Return the amount of variables in the expression
    pub fn var_count(&self) -> usize {
        self.var_count
    }
}

/// All the possible supported tokens in a
/// Boolean Expression
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<'a> {
    //
    // Values
    //
    /// A literal value, wheather `true` or `false`
    Literal(bool),
    /// A variable, with a given name and index.
    /// The latter of which is based on when
    /// the variable first appears in a given
    /// Boolean Expression
    Var(&'a str, usize),
    //
    // Operations
    //
    /// The NOT Operator
    Not,
    /// The AND Operator
    And,
    /// The eXclusiveOR Operator
    Xor,
    /// The OR Operator
    Or,
    /// The IMPLICATION Operator
    Implication,
    /// The EQUALITY Operator
    Equality,
    //
    // Extras
    //
    /// A Left Parenthesis
    LParen,
    /// A Right Parenthesis
    RParen,
}

impl<'a> Token<'a> {
    /// Determines Operator precedence
    ///
    /// |   Operator  | Precedence |
    /// | :---------: | :--------: |
    /// |     Not     |     5      |
    /// |     And     |     4      |
    /// |     Xor     |     3      |
    /// |      Or     |     2      |
    /// | Implication |     1      |
    /// |  Equality   |     0      |
    ///
    /// Any [`Token`] that is not an Operator
    /// has a precedence equal to
    /// [`isize::max_value()`].
    ///
    /// [`Token`]: `Token`
    /// [`isize::max_value()`]: `isize::max_value()`
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
/// for what each [`Token`] could look like
/// in an input string
///
/// [`Token`]: `Token`
#[derive(Debug, PartialEq)]
pub struct TokenLiterals {
    /// An array representing all the forms
    /// that the [`Token::Literal(true)`] value
    /// can take in any given Boolean Expression.
    ///
    /// [`Token::Literal(true)`]: `Token::Literal`
    pub lit_true: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Literal(false)`] value
    /// can take in any given Boolean Expression.
    ///
    /// [`Token::Literal(false)`]: `Token::Literal`
    pub lit_false: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Not`] token can
    /// take in any given Boolean Expression.
    ///
    /// [`Token::Not`]: `Token::Not`
    pub not: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::And`] token can
    /// take in any given Boolean Expression.
    ///
    /// [`Token::And`]: `Token::And`
    pub and: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Xor`] token can take
    /// in any given Boolean Expression.
    ///
    /// [`Token::Xor`]: `Token::Xor`
    pub xor: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Or`] token can
    /// take in any given Boolean Expression.
    ///
    /// [`Token::Or`]: `Token::Or`
    pub or: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Implication`] token
    /// can take in any given Boolean Expression.
    ///
    /// [`Token::Implication`]: `Token::Implication`
    pub implication: Vec<String>,
    /// An array representing all the forms
    /// that the [`Token::Equality`] token
    /// can take in any given Boolean Expression.
    ///
    /// [`Token::Equality`]: `Token::Equality`
    pub equality: Vec<String>,
    /// An array representing all the forms
    /// that a [`Token::LParen`] token can
    /// take in any given Boolean Expression.
    ///
    /// [`Token::LParen`]: `Token::LParen`
    pub left_paren: Vec<String>,
    /// An array representing all the forms
    /// that a [`Token::RParen`] token can
    /// take in any given Boolean Expression.
    ///
    /// [`Token::RParen`]: `Token::RParen`
    pub right_paren: Vec<String>,
}

/// The default set of values for the [`TokenLiterals`] struct
/// are as follows:
/// ```
/// # use truth_tester::parsing::TokenLiterals;
/// # assert_eq!(TokenLiterals::default(),
/// TokenLiterals {
///     lit_true: vec!["true"]
/// # .into_iter().map(Into::into).collect(),
///     lit_false: vec!["false"]
/// # .into_iter().map(Into::into).collect(),
///     not: vec!["¬", "not", "!", "~"]
/// # .into_iter().map(Into::into).collect(),
///     and: vec!["∧", "and", "&&", "&", "*"]
/// # .into_iter().map(Into::into).collect(),
///     xor: vec!["⊕", "xor", "^"]
/// # .into_iter().map(Into::into).collect(),
///     or: vec!["∨", "or", "||", "|", "+"]
/// # .into_iter().map(Into::into).collect(),
///     implication: vec!["→", "->", "=>"]
/// # .into_iter().map(Into::into).collect(),
///     equality: vec!["≡", "<=>", "==", "="]
/// # .into_iter().map(Into::into).collect(),
///     left_paren: vec!["(", "{", "["]
/// # .into_iter().map(Into::into).collect(),
///     right_paren: vec![")", "}", "]"]
/// # .into_iter().map(Into::into).collect(),
/// }
/// # );
/// ```
///
/// [`TokenLiterals`]: `TokenLiterals`
impl Default for TokenLiterals {
    fn default() -> Self {
        Self {
            lit_true: vec!["true"].into_iter().map(Into::into).collect(),
            lit_false: vec!["false"].into_iter().map(Into::into).collect(),
            not: vec!["¬", "not", "!", "~"]
                .into_iter()
                .map(Into::into)
                .collect(),
            and: vec!["∧", "and", "&&", "&", "*"]
                .into_iter()
                .map(Into::into)
                .collect(),
            xor: vec!["⊕", "xor", "^"].into_iter().map(Into::into).collect(),
            or: vec!["∨", "or", "||", "|", "+"]
                .into_iter()
                .map(Into::into)
                .collect(),
            implication: vec!["→", "->", "=>"].into_iter().map(Into::into).collect(),
            equality: vec!["≡", "<=>", "==", "="]
                .into_iter()
                .map(Into::into)
                .collect(),
            left_paren: vec!["(", "{", "["].into_iter().map(Into::into).collect(),
            right_paren: vec![")", "}", "]"].into_iter().map(Into::into).collect(),
        }
    }
}
