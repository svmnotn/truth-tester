use bit_field::BitField;

/// The state of a [`Tester`]
///
/// Oversized to contain the maximum amount of variables possible
/// in the architecture (32 in 32-bit, 64 in 64-bit).
///
/// This is due to the variables being solely stored inside a [`usize`]
///
/// [`Tester`]: `crate::tester::Tester`
/// [`usize`]: `usize`
#[derive(Clone, Copy)]
pub struct State {
    vars: usize,
    var_count: usize,
}

impl State {
    /// Create a new [`State`]
    ///
    /// [`State`]: `State`
    pub fn default(var_count: usize) -> Self {
        // Make sure that we will not overflow
        assert!(
            var_count < core::mem::size_of::<usize>() * 8,
            "Cannot have more variables than the width of a pointer"
        );

        Self { vars: 0, var_count }
    }

    /// Return the [`State`] at `iter`
    ///
    /// [`State`]: `State`
    pub fn iterate(&self, iteration: usize) -> Self {
        // because we use an unsingned integer to store
        // our bit values, every iteration represents a
        // different unique `State`.
        Self {
            vars: iteration,
            ..*self
        }
    }

    /// Returns the iteration represented
    /// by this [`State`].
    ///
    /// [`State`]: `State`
    pub fn iteration(&self) -> usize {
        self.vars
    }

    /// Return the variable at `idx`
    ///
    /// ## Panics
    /// This method will panic if `idx` >= `Self::var_count()`.
    pub fn var_at(&self, idx: usize) -> bool {
        assert!(
            idx <= self.var_count,
            "`idx` can not be greater than that amount of variables"
        );

        self.vars.get_bit(idx)
    }

    /// How many variables are managed by this [`State`]?
    ///
    /// [`State`]: `State`
    pub fn var_count(&self) -> usize {
        self.var_count
    }

    /// How many iterations can be handled by this [`State`]?
    ///
    /// [`State`]: `State`
    pub fn max_iters(&self) -> usize {
        1 << self.var_count
    }
}
