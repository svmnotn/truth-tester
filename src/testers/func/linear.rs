use crate::{ExprFn, State, Tester};

/// Linear implementation of all
/// the [`Tester`] methods,
/// based on an [`ExprFn`].
///
/// [`Tester`]: `Tester`
/// [`ExprFn`]: `ExprFn`
impl<E: ExprFn> Tester<E> {
    /// Checks if any of possible states of the `var_count`
    /// variables fails the given expression.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `expr`. And `false` otherwise.
    pub fn passes(var_count: usize, expr: E) -> bool {
        Self::new(var_count, expr).succeeded()
    }

    /// Checks if any of possible states of the `var_count`
    /// variables passes the given expression.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `expr`. And `false` otherwise.
    pub fn fails(var_count: usize, expr: E) -> bool {
        Self::new(var_count, expr).failed()
    }

    /// This returns `true` iff there are no failures
    pub fn succeeded(&self) -> bool {
        self.failures().any(|_| true) == false
    }

    /// This returns `true` iff there are no sucesses
    pub fn failed(&self) -> bool {
        self.successes().any(|_| true) == false
    }

    fn iterations(&self) -> impl Iterator<Item = usize> {
        0..self.state.max_iters()
    }

    /// Iterate over all the successes in sequence
    pub fn successes<'a>(&'a self) -> impl Iterator<Item = State> + 'a {
        self.eval()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    /// Iterate over all the failures in sequence
    pub fn failures<'a>(&'a self) -> impl Iterator<Item = State> + 'a {
        self.eval()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    /// Evaluate the expression of this [`Tester`]
    ///
    /// [`Tester`]: `Tester`
    pub fn eval<'a>(&'a self) -> impl Iterator<Item = (State, bool)> + 'a {
        self.iterations().map(move |iter| {
            let state = self.state.iterate(iter);
            (state, (self.expr)(&state))
        })
    }
}
