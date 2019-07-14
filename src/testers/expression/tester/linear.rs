use super::{Token, Tester};
use crate::State;
use alloc::vec::Vec;

impl<'t> Tester<'t> {
    /// Checks if any of possible states of all variables
    /// in the given expression fails the given expression.
    ///
    /// This function returns `true` if all possible states pass
    /// the given `expr`. And `false` otherwise.
    pub fn passes<'i: 't>(inp: &'i str) -> bool {
        Tester::parse(inp).succeeded()
    }

    /// Checks if any of possible states of all variables
    /// in the given expression passes the given expression.
    ///
    /// This function returns `true` if all possible states fail
    /// the given `expr`. And `false` otherwise.
    pub fn fails<'i: 't>(inp: &'i str) -> bool {
        Tester::parse(inp).failed()
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
    pub fn successes<'b>(&'b self) -> impl Iterator<Item = State> + 'b {
        self.test(true)
    }

    /// Iterate over all the failures in sequence
    pub fn failures<'b>(&'b self) -> impl Iterator<Item = State> + 'b {
        self.test(false)
    }

    fn test<'b>(&'b self, test: bool) -> impl Iterator<Item = State> + 'b {
        self.iterations().filter_map(move |iter| {
            use Token::*;

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
