//! Here are all the testing related things
//!
#![cfg_attr(feature = "parsing", doc(include = "tester/parsing.md"))]
//!
//! ### Using `truth_tester` with a function
//! To use `truth_tester` with a programatically defined function
//! instead of with a `&str`, all we have to do is change the
//! function use to create the [`Tester`] from [`Tester::parse`]
//! to [`Tester::new`].
//!
//! So our previous example becomes
//! ```
//! use truth_tester::tester::{Tester, State};
//!
//! // Create our tester with a given boolean expression
//! let tester = Tester::new(2, |s: &State| s.var_at(0) && s.var_at(1));
//!
//! // evaluate the expression at each possible state
//! for (state, val) in tester.eval() {
//!     println!("Iteration: {} of our expression has the following variables", state.iteration());
//!     for i in 0..state.var_count() {
//!         println!("Var: {}, with value {}", i, state.var_at(i));
//!     }
//!     println!("The result of our expression in this iteration is {}", val);
//! }
//! ```
//! 
//! [`Tester`]: `crate::tester::Tester`
#![cfg_attr(feature = "parsing", doc = "[`Tester::parse`]: `crate::tester::Tester::parse`")]
//! [`Tester::new`]: `crate::tester::Tester::new`
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
