use super::Tester;
use crate::State;

impl<E> Tester<E>
where
    E: Fn(&State) -> bool,
{
    /// Checks if any of possible states of the `var_count`
    /// variables fails the given expression.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `expr`. And `false` otherwise.
    pub fn passes(var_count: usize, expr: E) -> bool {
        Tester::new(var_count, expr).succeeded()
    }

    /// Checks if any of possible states of the `var_count`
    /// variables passes the given expression.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `expr`. And `false` otherwise.
    pub fn fails(var_count: usize, expr: E) -> bool {
        Tester::new(var_count, expr).failed()
    }

    /// This returns `true` iff there are no failures
    pub fn succeeded(&self) -> bool {
        !self.failures().any(|_| true)
    }

    /// This returns `true` iff there are no sucesses
    pub fn failed(&self) -> bool {
        !self.successes().any(|_| true)
    }

    fn iterations(&self) -> impl Iterator<Item = usize> {
        0..self.state.max_iters()
    }

    /// Iterate over all the successes in sequence
    pub fn successes<'a>(&'a self) -> impl Iterator<Item = State> + 'a {
        self.test(true)
    }

    /// Iterate over all the failures in sequence
    pub fn failures<'a>(&'a self) -> impl Iterator<Item = State> + 'a {
        self.test(false)
    }

    fn test<'a>(&'a self, test: bool) -> impl Iterator<Item = State> + 'a {
        self.iterations().filter_map(move |iter| {
            let state = self.state.iterate(iter);
            if (self.expr)(&state) == test {
                Some(state)
            } else {
                None
            }
        })
    }

    /// Get the full truth table
    #[cfg(feature = "alloc")]
    pub fn all(&self) -> alloc::vec::Vec<(State, bool)> {
        self.iterations()
            .map(move |iter| {
                let s = self.state.iterate(iter);
                let v = (self.expr)(&s);
                (s, v)
            })
            .collect()
    }

    /// Get a table of all states where 
    /// the `expr` succeeds.
    #[cfg(feature = "alloc")]
    pub fn all_successes(&self) -> alloc::vec::Vec<State> {
        self.successes().collect()
    }

    /// Get a table of all states where 
    /// the `expr` fails.
    #[cfg(feature = "alloc")]
    pub fn all_failures(&self) -> alloc::vec::Vec<State> {
        self.failures().collect()
    }
}
