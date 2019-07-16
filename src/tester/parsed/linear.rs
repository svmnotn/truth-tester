use crate::{
    parsing::Tokens,
    tester::{State, Tester},
};

/// Linear implementation of all
/// the [`Tester`] methods,
/// based on parsed [`Tokens`].
///
/// [`Tester`]: `Tester`
/// [`Tokens`]: `Tokens`
impl<'t> Tester<Tokens<'t>> {
    /// This returns `true` iff there are no failures
    pub fn is_true(&self) -> bool {
        self.failures().any(|_| true) == false
    }

    /// This returns `true` iff there are no sucesses
    pub fn is_false(&self) -> bool {
        self.successes().any(|_| true) == false
    }

    fn iterations(&self) -> impl Iterator<Item = usize> {
        0..self.state.max_iters()
    }

    /// Iterate over all the successes in sequence
    pub fn successes<'b>(&'b self) -> impl Iterator<Item = State> + 'b {
        self.eval()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    /// Iterate over all the failures in sequence
    pub fn failures<'b>(&'b self) -> impl Iterator<Item = State> + 'b {
        self.eval()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    /// Evaluate the expression of this [`Tester`]
    ///
    /// [`Tester`]: `Tester`
    pub fn eval<'b>(&'b self) -> impl Iterator<Item = (State, bool)> + 'b {
        self.iterations().map(move |iter| self.eval_iter(iter))
    }
}
