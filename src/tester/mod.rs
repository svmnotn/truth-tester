//! Here are all the testing related things
mod func;
pub use func::ExprFn;

#[cfg(feature = "parsing")]
mod parsed;

mod expr;
use expr::Expression;

mod state;
pub use state::State;

/// A struct used to store both
/// the user given expression,
/// and the [`State`] of that expression.
///
/// [`State`]: `State`
pub struct Tester<E: Expression> {
    state: State,
    expr: E,
}
