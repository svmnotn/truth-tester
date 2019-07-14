/// A full expression, which is made of [`Token`]s
/// 
/// [`Token`]: `Token`
pub type Tokens<'a> = alloc::vec::Vec<Token<'a>>;

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
    /// The end of the given input was reached.
    /// It contains a count of how many variables
    /// where inside the expression.
    EOF(usize),
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
#[derive(Debug)]
pub struct TokenLiterals<'a> {
    /// An array representing all the forms
    /// that the [`Token::Literal(true)`] value
    /// can take in any given Boolean Expression.
    /// 
    /// [`Token::Literal(true)`]: `Token::Literal`
    pub lit_true: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Literal(false)`] value
    /// can take in any given Boolean Expression.
    /// 
    /// [`Token::Literal(false)`]: `Token::Literal`
    pub lit_false: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Not`] token can 
    /// take in any given Boolean Expression.
    /// 
    /// [`Token::Not`]: `Token::Not`
    pub not: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::And`] token can 
    /// take in any given Boolean Expression.
    /// 
    /// [`Token::And`]: `Token::And`
    pub and: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Xor`] token can take 
    /// in any given Boolean Expression.
    /// 
    /// [`Token::Xor`]: `Token::Xor`
    pub xor: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Or`] token can 
    /// take in any given Boolean Expression.
    /// 
    /// [`Token::Or`]: `Token::Or`
    pub or: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Implication`] token 
    /// can take in any given Boolean Expression.
    /// 
    /// [`Token::Implication`]: `Token::Implication`
    pub implication: &'a [&'a str],
    /// An array representing all the forms
    /// that the [`Token::Equality`] token 
    /// can take in any given Boolean Expression.
    /// 
    /// [`Token::Equality`]: `Token::Equality`
    pub equality: &'a [&'a str],
    /// An array representing all the forms
    /// that a [`Token::LParen`] token can 
    /// take in any given Boolean Expression.
    /// 
    /// [`Token::LParen`]: `Token::LParen`
    pub left_paren: &'a [&'a str],
    /// An array representing all the forms
    /// that a [`Token::RParen`] token can 
    /// take in any given Boolean Expression.
    /// 
    /// [`Token::RParen`]: `Token::RParen`
    pub right_paren: &'a [&'a str],
}

/// The default set of values for the [`TokenLiterals`] struct
/// are as follows:
/// ```rust
/// TokenLiterals {
///     lit_true: &["true"],
///     lit_false: &["false"],
///     not: &["¬", "not", "!", "~"],
///     and: &["∧", "and", "&&", "&", "*"],
///     xor: &["⊕", "xor", "^"],
///     or: &["∨", "or", "||", "|", "+"],
///     implication: &["→", "->", "=>"],
///     equality: &["≡", "<=>", "==", "="],
///     left_paren: &["(", "{", "["],
///     right_paren: &[")", "}", "]"],
/// }
/// ```
/// 
/// [`TokenLiterals`]: `TokenLiterals`
impl Default for TokenLiterals<'static> {
    fn default() -> Self {
        Self {
            lit_true: &["true"],
            lit_false: &["false"],
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
