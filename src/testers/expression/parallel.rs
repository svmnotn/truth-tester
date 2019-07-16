use crate::{parsing::Tokens, State, Tester};
use rayon::prelude::*;

/// Parallel implementation of all
/// the [`Tester`] methods,
/// based on parsed [`Tokens`].
///
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// This returns `true` iff there are no failures
    ///
    /// This function is the parallel version of [`Tester::is_true`]
    ///
    /// [`Tester::is_true`]: `Tester::is_true`
    pub fn is_true_par(&self) -> bool {
        self.failures_par().any(|_| true) == false
    }

    /// This returns `true` iff there are no sucesses
    ///
    /// This function is the parallel version of [`Tester::is_false`]
    ///
    /// [`Tester::is_false`]: `Tester::is_false`
    pub fn is_false_par(&self) -> bool {
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
    pub fn successes_par<'b>(&'b self) -> impl ParallelIterator<Item = State> + 'b {
        self.eval_par()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    /// Iterate over all the failures in parallel
    ///
    /// This function is the parallel version of [`Tester::failures`]
    ///
    /// [`Tester::failures`]: `Tester::failures`
    pub fn failures_par<'b>(&'b self) -> impl ParallelIterator<Item = State> + 'b {
        self.eval_par()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    /// Evaluate the expression of this [`Tester`]
    ///
    /// This function is the parallel version of [`Tester::eval`]
    ///
    /// [`Tester`]: `Tester`
    /// [`Tester::eval`]: `Tester::eval`
    pub fn eval_par<'b>(&'b self) -> impl ParallelIterator<Item = (State, bool)> + 'b {
        self.iterations_par().map(move |iter| self.eval_iter(iter))
    }
}
