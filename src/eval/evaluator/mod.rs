#[cfg(feature = "parsing")]
mod parsed;

mod linear;
#[cfg(feature = "parallel")]
mod parallel;

use crate::eval::{Expression, State};
use core::ops::Range;

pub struct Tester<E: Expression> {
    pub(crate) expr: E,
    pub(crate) var_count: usize,
}

impl<E: Expression> Tester<E> {
    pub fn new<T: Into<E>>(expr: T, var_count: usize) -> Self {
        Self {
            expr: expr.into(),
            var_count,
        }
    }

    pub fn var_vals<'a, S: State + 'a>(&'a self, state: S) -> impl Iterator<Item = bool> + 'a {
        (0..self.var_count).map(move |v| state.var_at(v))
    }

    pub fn var_count(&self) -> usize {
        self.var_count
    }

    pub fn max_iter(&self) -> usize {
        1 << self.var_count
    }

    pub(crate) fn iterations(&self) -> Range<usize> {
        0..self.max_iter()
    }
}
