# Rua

CS2612 PL&amp;Compilers Final Project: an interpreter for a lua subset.

- `./syntax.md`: syntax for the Lua subset

- `./doc.md`: usage for the Lua subset

- `./impl.md`: some rough implementation details for the interpreter 

- `./src/` source files

- `./tests/` some test files

## Compile and Run

```
cargo run <lua-file>
```

For example, enter `cargo run ./tests/assignment.lua` and get the following output: 
```
1       2       3
100     200     nil
0       1       2       3       nil
4       20      4
20      30      10
```

The code is only tested on windows. It might also work on linux.
