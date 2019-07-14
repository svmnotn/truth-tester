mod func;
pub use func::ExprFn;

#[cfg(feature = "alloc")]
mod expression;

/// A Trait representing all possible
/// types that can hold an Expression
/// that can be used inside a [`Tester`].
///
/// [`Tester`]: `Tester`
pub trait Expression {}

impl<E: ExprFn> Expression for E {}

#[cfg(feature = "alloc")]
use crate::parsing::Tokens;

#[cfg(feature = "alloc")]
impl<'a> Expression for Tokens<'a> {}

use crate::State;

/// A struct used to store both
/// the user given expression,
/// and the [`State`] of that expression.
///
/// [`State`]: `State`
pub struct Tester<E: Expression> {
    state: State,
    expr: E,
}
