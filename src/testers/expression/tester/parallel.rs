use super::{Token, Tokens, Tester};
use crate::State;
use rayon::prelude::*;

impl<'t> Tester<'t> {
    /// Checks if any of possible states of all variables
    /// in the given expression fails the given expression.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `expr`. And `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::passes`]
    /// 
    /// [`Tester::passes`]: `Tester::passes`
    pub fn passes_par(expr: Tokens<'t>) -> bool {
        Tester::new(expr).succeeded_par()
    }

    /// Checks if any of possible states of all variables
    /// in the given expression passes the given expression.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `expr`. And `false` otherwise.
    ///
    /// This function is the parallel version of [`Tester::fails`]
    /// 
    /// [`Tester::fails`]: `Tester::fails`
    pub fn fails_par(expr: Tokens<'t>) -> bool {
        Tester::new(expr).failed_par()
    }

    /// This returns `true` iff there are no failures
    ///
    /// This function is the parallel version of [`Tester::succeeded`]
    /// 
    /// [`Tester::succeeded`]: `Tester::succeeded`
    pub fn succeeded_par(&self) -> bool {
        !self.failures_par().any(|_| true)
    }

    /// This returns `true` iff there are no sucesses
    ///
    /// This function is the parallel version of [`Tester::failed`]
    /// 
    /// [`Tester::failed`]: `Tester::failed`
    pub fn failed_par(&self) -> bool {
        !self.successes_par().any(|_| true)
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
        self.test_par(true)
    }

    /// Iterate over all the failures in parallel
    /// 
    /// This function is the parallel version of [`Tester::failures`]
    /// 
    /// [`Tester::failures`]: `Tester::failures`
    pub fn failures_par<'b>(&'b self) -> impl ParallelIterator<Item = State> + 'b {
        self.test_par(false)
    }

    fn test_par<'b>(&'b self, test: bool) -> impl ParallelIterator<Item = State> + 'b {
        self.iterations_par().filter_map(move |iter| {
            use Token::*;
            use alloc::vec::Vec;

            let state = self.state.iterate(iter);
            let mut stack: Vec<bool> = Vec::new();
            // turn the expression into a single value
            for t in &self.expr {
                match *t {
                    // Values can go directly to the stack
                    Var(_, v) => stack.push(state.var_at(v)),
                    Literal(v) => stack.push(v),
                    // Operations
                    Not => {
                        if let Some(v) = stack.pop() {
                            stack.push(!v);
                        }
                    }
                    And => {
                        if let Some(b) = stack.pop() {
                            if let Some(a) = stack.pop() {
                                stack.push(a && b);
                            }
                        }
                    }
                    Xor => {
                        if let Some(b) = stack.pop() {
                            if let Some(a) = stack.pop() {
                                stack.push(a ^ b);
                            }
                        }
                    }
                    Or => {
                        if let Some(b) = stack.pop() {
                            if let Some(a) = stack.pop() {
                                stack.push(a || b);
                            }
                        }
                    }
                    Implication => {
                        if let Some(b) = stack.pop() {
                            if let Some(a) = stack.pop() {
                                stack.push(!a || b);
                            }
                        }
                    }
                    Equality => {
                        if let Some(b) = stack.pop() {
                            if let Some(a) = stack.pop() {
                                stack.push(a == b);
                            }
                        }
                    }
                    EOF(_) => unreachable!(),
                    LParen => unreachable!(),
                    RParen => unreachable!(),
                }
            }

            if stack.len() != 1 {
                panic!("The expression did not resove to a single value!");
            }

            if let Some(v) = stack.pop() {
                if v == test {
                    Some(state)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}
