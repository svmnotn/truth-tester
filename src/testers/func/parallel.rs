use crate::{ExprFn, State, Tester};
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
        self.eval_par()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    /// Iterate over all the failures in parallel
    ///
    /// This function is the parallel version of [`Tester::failures`]
    ///
    /// [`Tester::failures`]: `Tester::failures`
    pub fn failures_par<'a>(&'a self) -> impl ParallelIterator<Item = State> + 'a {
        self.eval_par()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    /// Evaluate the expression of this [`Tester`]
    ///
    /// This function is the parallel version of [`Tester::eval`]
    ///
    /// [`Tester`]: `Tester`
    /// [`Tester::eval`]: `Tester::eval`
    fn eval_par<'a>(&'a self) -> impl ParallelIterator<Item = (State, bool)> + 'a {
        self.iterations_par().map(move |iter| {
            let state = self.state.iterate(iter);
            (state, (self.expr)(&state))
        })
    }
}
