# S-Interpreter
A dead simple interpreter for the dead simple language of S. Powered by Rust.

See `add.s` for an example program. The syntax is like brainfxck in assembly.

`Xn` is an input variable, from the command line. (natural numbers only)

`Zn` are any local variables

`Y` is the return value

`X` or `Z` with no number is equivalent to `X1` or `Z1`

Labels are any capital letter except for variables, followed by a number.

Add a label to a line with something like `[A1]:`

Label `E` is by convention reserved for exiting the program.


Build with `cargo build`

Run with `cargo run <source file> [params...]`
