use crate::{parsing::Tokens, State, Tester};
use rayon::prelude::*;

/// Parallel implementation of all
/// the [`Tester`] methods,
/// based on parsed [`Tokens`].
///
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// Checks if any of possible states of all variables
    /// in the given input expression fails.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `inp`, and `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::passes`]
    ///
    /// [`Tester::passes`]: `Tester::passes`
    pub fn passes_par<'i: 't>(inp: &'i str) -> bool {
        Self::parse(inp).succeeded_par()
    }

    /// Checks if any of possible states of all variables
    /// in the given input expression passes.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `inp`, and `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::fails`]
    ///
    /// [`Tester::fails`]: `Tester::fails`
    pub fn fails_par<'i: 't>(inp: &'i str) -> bool {
        Self::parse(inp).failed_par()
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
