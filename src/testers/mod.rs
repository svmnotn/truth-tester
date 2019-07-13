//! Here live all the Testers
mod func;
pub use func::Tester as FnTester;

#[cfg(feature = "alloc")]
pub mod expression;
#[cfg(feature = "alloc")]
pub use expression::ExprTester;
