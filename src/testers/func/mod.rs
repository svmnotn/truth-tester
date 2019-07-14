mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::State;

/// A function that takes a [`State`] and returns a [`bool`]
/// 
/// Normally something along the lines of
/// `|s: &State| s.var_at(0) && s.var_at(1)`
/// 
/// [`bool`]: `bool`
/// [`State`]: `State`
pub trait ExprFn = Fn(&State) -> bool;

/// Use this type to test any function
/// of the form `fn(&State) -> bool`
pub struct Tester<E>
where
    E: ExprFn,
{
    state: State,
    expr: E,
}

/// [`Tester`] based on an [`ExprFn`].
/// 
/// [`Tester`]: `Tester`
/// [`ExprFn`]: `ExprFn`
impl<E: ExprFn> Tester<E> {
    /// Create a new [`Tester`] with a given [`ExprFn`].
    /// 
    /// [`Tester`]: `Tester`
    /// [`ExprFn`]: `ExprFn`
    pub fn new(var_count: usize, expr: E) -> Tester<E> {
        Tester {
            state: State::default(var_count),
            expr,
        }
    }
}
