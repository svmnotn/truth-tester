mod linear;

#[cfg(feature = "parallel")]
mod parallel;

use crate::State;

/// Use this type to test any function
/// of the form `fn(&State) -> bool`
pub struct Tester<E>
where
    E: Fn(&State) -> bool,
{
    state: State,
    expr: E,
}

impl<E> Tester<E>
where
    E: Fn(&State) -> bool,
{
    /// Create a new `Tester`
    pub fn new(var_count: usize, expr: E) -> Tester<E> {
        Tester {
            state: State::default(var_count),
            expr,
        }
    }
}
