/// A Trait representing all possible
/// types that can hold an Expression
/// that can be used inside a [`Tester`].
///
/// [`Tester`]: `Tester`
pub trait Expression {}

use crate::tester::ExprFn;
impl<E: ExprFn> Expression for E {}

#[cfg(feature = "parsing")]
use crate::parsing::Tokens;

#[cfg(feature = "parsing")]
impl<'a> Expression for Tokens<'a> {}
