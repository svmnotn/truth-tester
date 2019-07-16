use crate::{parsing::Tokens, State, Tester};

/// Linear implementation of all
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
    pub fn passes<'i: 't>(inp: &'i str) -> bool {
        Self::parse(inp).succeeded()
    }

    /// Checks if any of possible states of all variables
    /// in the given input expression passes.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `inp`, and `false` otherwise.
    pub fn fails<'i: 't>(inp: &'i str) -> bool {
        Self::parse(inp).failed()
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
