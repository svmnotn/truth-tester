//! `truth_tester` is a library for testing boolean expressions.
//! The expressions can be in one of two froms:
//! - a function of type `fn(&State) -> bool`
//! and the amount of variables in it
//! - a `&str` with a boolean expression.
//!
//! Furthermore, as `truth_tester` is able to interpret a boolean
//! expression, the parser it uses is also available under the
//! [`parsing`] module.
//!
//! ## Using `truth_tester` for its Testing capabilities
//!
//! For the complete guide check the [`tester`] module docs.
//!
//! But be aware that if you disable default features,
//! by specifying so in your Cargo.toml
//! ```toml
//! [dependencies.truth_tester]
//! version = "0.1.0"
//! default-features = false
//! ```
//! you then have to enable the `tester` feature
//! ```toml
//! [dependencies.truth_tester]
//! version = "0.1.0"
//! default-features = false
//! features = ["tester"]
//! ```
//! As it is needed for the [`tester`] module to be
//! included in the code.
//!
//! For testing parsed expressions, the `parsing` feature
//! also needs to be enabled.
//! ```toml
//! [dependencies.truth_tester]
//! version = "0.1.0"
//! default-features = false
//! features = ["tester", "parsing"]
//! ```
//!
//! ## Using `truth_tester` as a boolean expression parser
//!
//! For the complete guide check the [`parsing`] module docs.
//!
//! But be aware that if you disable default features,
//! by specifying so in your Cargo.toml
//! ```toml
//! [dependencies.truth_tester]
//! version = "0.1.0"
//! default-features = false
//! ```
//! you then have to enable the `parsing` feature
//! ```toml
//! [dependencies.truth_tester]
//! version = "0.1.0"
//! default-features = false
//! features = ["parsing"]
//! ```
//! As it is needed for the [`parsing`] module to be
//! included in the code.
//!
#![cfg_attr(feature = "parsing", doc = "[`parsing`]: `parsing`")]
#![cfg_attr(feature = "tester", doc = "[`tester`]: `tester`")]
#![no_std]
#![warn(missing_docs)]
// This is used in order to have `ExprFn` exist,
// can be removed once `trait_alias` stabilizes,
// see https://github.com/rust-lang/rust/issues/41517
#![feature(trait_alias)]
// This is for conditional compilation of code examples
#![feature(external_doc)]

#[cfg(feature = "parsing")]
extern crate alloc;

#[cfg(feature = "tester")]
pub mod tester;

#[cfg(feature = "parsing")]
pub mod parsing;
