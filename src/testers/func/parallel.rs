use crate::{Tester, ExprFn, State};
use rayon::prelude::*;

/// Parallel implementation of all 
/// the [`Tester`] methods, 
/// based on an [`ExprFn`].
/// 
/// [`Tester`]: `Tester`
/// [`ExprFn`]: `ExprFn`
impl<E> Tester<E>
where
    E: ExprFn + Send + Sync,
{
    /// Checks if any of possible states of the `var_count`
    /// variables fails the given expression.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `expr`. And `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::passes`]
    /// 
    /// [`Tester::passes`]: `Tester::passes`
    pub fn passes_par(var_count: usize, expr: E) -> bool {
        Self::new(var_count, expr).succeeded_par()
    }

    /// Checks if any of possible states of the `var_count`
    /// variables passes the given expression.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `expr`. And `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::fails`]
    /// 
    /// [`Tester::fails`]: `Tester::fails`
    pub fn fails_par(var_count: usize, expr: E) -> bool {
        Self::new(var_count, expr).failed_par()
    }

    /// This returns `true` iff there are no failures
    ///
    /// This function is the parallel version of [`Tester::succeeded`]
    /// 
    /// [`Tester::succeeded`]: `Tester::succeeded`
    pub fn succeeded_par(&self) -> bool {
        self.failures_par().any(|_| true) == false
    }

    /// This returns `true` iff there are no sucesses
    ///
    /// This function is the parallel version of [`Tester::failed`]
    /// 
    /// [`Tester::failed`]: `Tester::failed`
    pub fn failed_par(&self) -> bool {
        self.successes_par().any(|_| true) == false
    }

    fn iterations_par(&self) -> impl ParallelIterator<Item = usize> {
        (0..self.state.max_iters()).into_par_iter()
    }

    /// Iterate over all the successes in parallel
    /// 
    /// This function is the parallel version of [`Tester::successes`]
    /// 
    /// [`Tester::successes`]: `Tester::successes`
    pub fn successes_par<'a>(&'a self) -> impl ParallelIterator<Item = State> + 'a {
        self.test_par(true)
    }

    /// Iterate over all the failures in parallel
    /// 
    /// This function is the parallel version of [`Tester::failures`]
    /// 
    /// [`Tester::failures`]: `Tester::failures`
    pub fn failures_par<'a>(&'a self) -> impl ParallelIterator<Item = State> + 'a {
        self.test_par(false)
    }

    fn test_par<'a>(&'a self, test: bool) -> impl ParallelIterator<Item = State> + 'a {
        self.iterations_par().filter_map(move |iter| {
            let s = self.state.iterate(iter);
            if (self.expr)(&s) == test {
                Some(s)
            } else {
                None
            }
        })
    }

    /// Get the full truth table
    /// 
    /// This function is the parallel version of [`Tester::all`]
    /// 
    /// [`Tester::all`]: `Tester::all`
    #[cfg(feature = "alloc")]
    pub fn all_par(&self) -> alloc::vec::Vec<(State, bool)> {
        self.iterations_par()
            .map(move |iter| {
                let s = self.state.iterate(iter);
                let v = (self.expr)(&s);
                (s, v)
            })
            .collect()
    }

    /// Get a table of all states where 
    /// the `expr` succeeds.
    /// 
    /// This function is the parallel version of [`Tester::all_successes`]
    /// 
    /// [`Tester::all_successes`]: `Tester::all_successes`
    #[cfg(feature = "alloc")]
    pub fn all_successes_par(&self) -> alloc::vec::Vec<State> {
        self.successes_par().collect()
    }

    /// Get a table of all states where 
    /// the `expr` fails.
    /// 
    /// This function is the parallel version of [`Tester::all_failures`]
    /// 
    /// [`Tester::all_failures`]: `Tester::all_failures`
    #[cfg(feature = "alloc")]
    pub fn all_failures_par(&self) -> alloc::vec::Vec<State> {
        self.failures_par().collect()
    }
}
