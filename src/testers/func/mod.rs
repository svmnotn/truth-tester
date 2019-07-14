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

impl<E> Tester<E>
where
    E: ExprFn,
{
    /// Create a new `Tester`
    pub fn new(var_count: usize, expr: E) -> Tester<E> {
        Tester {
            state: State::default(var_count),
            expr,
        }
    }
}
