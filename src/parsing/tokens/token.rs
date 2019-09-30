/// All the possible supported tokens in a
/// Boolean Expression
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    /// [`usize::max_value()`].
    ///
    /// [`Token`]: `Token`
    /// [`isize::max_value()`]: `usize::max_value()`
    pub fn precedence(&self) -> usize {
        match self {
            Self::Not => 5,
            Self::And => 4,
            Self::Xor => 3,
            Self::Or => 2,
            Self::Implication => 1,
            Self::Equality => 0,
            _ => usize::max_value(),
        }
    }
}
