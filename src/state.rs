use bit_field::BitField;
// This is an import for the sake
// of the docs
#[allow(unused_imports)]
use crate::Tester;

/// The state of a [`Tester`]
/// 
/// Oversized to contain the maximum amount of variables possible
/// in the architecture (32 in 32-bit, 64 in 64-bit).
///
/// This is due to the variables being solely stored inside a [`usize`]
/// 
/// [`Tester`]: `Tester`
/// [`usize`]: `usize`
#[derive(Clone, Copy)]
pub struct State {
    vars: usize,
    var_count: usize,
}

impl State {
    /// Create a new `State`
    pub fn default(var_count: usize) -> Self {
        // Make sure that we will not overflow
        assert!(
            var_count < core::mem::size_of::<usize>() * 8,
            "Cannot have more variables than the width of a pointer"
        );

        State { vars: 0, var_count }
    }

    /// Return the state at `iter`
    pub fn iterate(&self, iteration: usize) -> Self {
        // because we use an unsingned integer to store
        // our bit values, every iteration represents a
        // different unique `State`.
        Self {
            vars: iteration,
            ..*self
        }
    }

    /// Return the variable at `idx`
    ///
    /// ## Panics
    /// This method will panic if `idx` >= `Self::var_count()`.
    pub fn var_at(&self, idx: usize) -> bool {
        assert!(
            idx <= self.var_count(),
            "`idx` can not be greater than that amount of variables"
        );

        self.vars.get_bit(idx)
    }

    /// How many variables are managed by this `State`?
    pub fn var_count(&self) -> usize {
        self.var_count
    }

    /// How many iterations can be handled by this `State`?
    pub fn max_iters(&self) -> usize {
        1 << self.var_count
    }
}
