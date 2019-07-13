mod linear;

#[cfg(feature = "parallel")]
mod parallel;

use crate::State;

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

    pub fn var_count(&self) -> usize {
        self.state.var_count()
    }
}
