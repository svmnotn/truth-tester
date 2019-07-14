mod func;
pub use func::ExprFn;

#[cfg(feature = "alloc")]
mod expression;

use crate::{State, parsing::Tokens};

/// A struct used to store both 
/// the user given expression,
/// and the [`State`] of that expression.
/// 
/// [`State`]: `State`
pub struct Tester<E: Expression> {
    state: State,
    expr: E,
}

/// A Trait representing all possible
/// types that can hold an Expression 
/// that can be used inside a [`Tester`].
/// 
/// [`Tester`]: `Tester`
pub trait Expression {}

#[cfg(feature = "alloc")]
impl<'a> Expression for Tokens<'a> {}
impl<E: ExprFn> Expression for E {}