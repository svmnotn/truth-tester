# truth-tester

A Work In Progress, `#![no_std]` compatible library that allows for the testing of boolean expressions in two ways:
1. Programmatically, using the library from another rust project you can give it a function along the lines of `|s: State| s.var_at(0) && s.var_at(1)` and it will test it for all possible values of that two variable state.
2. Via passing of a string containing a boolean expression. The expression is parsed and processed to obtain the results.
