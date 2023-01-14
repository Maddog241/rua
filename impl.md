# Main concepts in this interpreter

The interpreter can be split into 3 parts:

- lexer: get the source and emit tokens

- parser: get tokens and emit ast nodes

- interpreter: get an ast node and 
  - execute statements
  - evaluate expressions

There are some other files:

- `./src/main.rs` is the entry for execution
- `./src/rua.rs` defines a Rua object
- `./src/token.rs` defines all the tokens. A `Token` consists of `TokenType` and `line`. The former is what we call 'token' in textbooks, and the latter is for better debug information.
- `./src/ast.rs` defines all the abstract syntax tree nodes, which include statements, expressions and some other helper structures. 
- `./src/value.rs` defines the runtime representation of the Lua values. 
  
  There are 6 types in our Lua Subset: nil, boolean, string, number, function, table. 
  
  A `Value` can be `Nil`, `Bool`, `Str`, `Num`, `Address`, `ValueList` and `Print`. 
  
  `Address` is used to dereference `HeapObj`, which contains `Function` and `Table`.
  
  `ValueList` is only used for function's return values. It can be expanded when it is the trailing expression in an assignment statement's 'explist'. Also, it can be expanded when it is the last field in the table constructor. Under other circumstances, `ValueList` should be considered as a single value, so the method `compress` will be called when needed.  
  
  `Print` is used for the builtin function `print`.

- `./src/environment.rs` defines the structure `Environment`. It is used to emulate Lua's stack frame. When the program enters a block, a new `Environment` object will be pushed onto the `Interpreters`'s `env_stack` field. When it exits a block, the `Environment` object will be poped. Global variables are defined in the bottommost `Environment`, Local variables are defined in the topmost `Environment`. 

For detailed implementation explanations, please see the comments in the source files. We think they are enough.