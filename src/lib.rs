mod tester;
pub use tester::Tester;

mod state;
pub use state::{Bool as BoolState, State};

/// Test the given `expr` on all posible binary states of `n` variables.
///
/// Returns `false` if the given expression ever becomes untrue.
pub fn test_xpr(n: usize, expr: &Fn(&[bool]) -> bool) -> bool {
    let mut tester = Tester::new(n, |v: &BoolState| expr(v.as_ref()));

    !tester.test_fail()
}
