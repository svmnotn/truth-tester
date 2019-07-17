### Using `truth_tester` with a `&str`
Below we simply evaluate the expression given at all
possible states, and print what we get from it
```
use truth_tester::tester::Tester;

// Create our tester with a given boolean expression
let tester = Tester::parse("A && B");

// evaluate the expression at each possible state
for (state, val) in tester.eval() {
    println!("Iteration: {} of our expression has the following variables", state.iteration());
    for i in 0..state.var_count() {
        let (var_name, var_value) = tester.var_at(&state, i);
        println!("Var: {}, with value {}", var_name, var_value);
    }
    println!("The result of our expression in this iteration is {}", val);
}
```

But we could do some more interesting thigs, like testing if an expression always succeeds
and thus resolves to `true`
```
use truth_tester::tester::Tester;

// Ask the tester we create if the expression we
// give it is always true
assert!(true, Tester::parse("A || true").is_true());
```
