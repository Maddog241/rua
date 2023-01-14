# Doc for Lua Subset - Rua

## Lexical Convention

### keywords

```
and break do else elseif 
end false for function if 
in  local nil not or 
return then true while
```

### strings

#### short string

Short strings can be delimited by matching single or duobel quotes, and can containg the following escape sequences: `\n, \t, \\, \', \"`.

#### long string

Literal strings can also be defined using a long format enclosed by double brackets: `[[<string-literal>]]`. However, it does not interpret any escape sequences. 

For convenience, when the opening double bracket is immediately followed by a newline, the newline is not included in the string. So the following two strings are equal: 

```lua
a = 'alo\n123"'
a = [[alo
123"]]
a = [[
alo
123"]]
```

### numbers

a numeric constant can be written with an optional fractional part. So both `333` and `333.33` are valid numbers.

### comments

A comment starts with a double hyphen (--) anywhere outside a string. If  the text immediately after `--` is not an opening double bracket, the comment is a *short comment*, which runs until the end of the line. Otherwise, it is a *long comment*, which runs until the corresponding closing double bracket. 

## Variables

Variables are places that store values. There are three kinds of variables in Lua: global variables, local variables, and table fields.

A single name can denote a global variable or a local variable (or a function's formal parameter, which is a particular kind of local variable):

    var ::= Name

Any variable name is assumed to be global unless explicitly declared as a local. Local variables are *lexically scoped*: local variables can be freely accessed by functions defined inside their scope.

Before the first assignment to a variable, its value is **nil**.

Square brackets are used to index a table:

    var ::= prefixexp '[' exp ']'

The syntax `var.Name` is just syntactic sugar for `var["Name"]`:

    var ::= prefixexp '.' Name

## Statements

### blocks

A block is a list of statements, which are executed sequentially:

    block ::= {stat}

Lua has *empty statements* that allow you to separate statements with semicolons, start a block with a semicolon or write two semicolons in sequence:

    stat ::= ';'

Function calls and assignments can start with an open parenthesis. This possibility leads to an ambiguity in Lua's grammar. Consider the following fragment:

     a = b + c
     (print or io.write)('done')

The grammar could see it in two ways:

     a = b + c(print)('done')
    
     a = b + c; (print)('done')

The current parser always sees such constructions in the first way, interpreting the open parenthesis as the start of the arguments to a call. To avoid this ambiguity, it is a good practice to always precede with a semicolon statements that start with a parenthesis:

     ;(print)('done')

A block can be explicitly delimited to produce a single statement:

    stat ::= do block end

Explicit blocks are useful to control the scope of variable declarations. Explicit blocks are also sometimes used to add a **return** statement in the middle of another block.

### assignment

Lua allows multiple assignments. Therefore, the syntax for assignment defines a list of variables on the left side and a list of expressions on the right side. The elements in both lists are separated by commas:

    stat ::= varlist '=' explist
    varlist ::= var {',' var}
    explist ::= exp {',' exp}

Before the assignment, the list of values is *adjusted* to the length of the list of variables. If there are more values than needed, the excess values are thrown away. If there are fewer values than needed, the list is extended with as many **nil**'s as needed. If the list of expressions ends with a function call, then all values returned by that call enter the list of values, before the adjustment.

The assignment statement first evaluates all its expressions and only then the assignments are performed. Thus the code

     i = 3
     i, a[i] = i+1, 20

sets `a[3]` to 20, without affecting `a[4]` because the `i` in `a[i]` is evaluated (to 3) before it is assigned 4. Similarly, the line

     x, y = y, x

exchanges the values of `x` and `y`, and

     x, y, z = y, z, x

cyclically permutes the values of `x`, `y`, and `z`.

### control structures

The control structures are **if**, **while** and **for** statements.

    stat ::= while exp do block end
    stat ::= if exp then block {elseif exp then block} [else block] end

**for** statements has two flavors, see the next section.

The condition expression of a control structure can return any value. Both **false** and **nil** are considered false. All values different from **nil** and **false** are considered true (in particular, the number 0 and the empty string are also true).

The **break** statement terminates the execution of a **while**, **repeat**, or **for** loop, skipping to the next statement after the loop:

    stat ::= break

A **break** ends the innermost enclosing loop.

The **return** statement is used to return values from a function. Functions can return more than one value, so the syntax for the **return** statement is

    stat ::= return [explist] [';']

The **return** statement can only be written as the last statement of a block. If it is really necessary to **return** in the middle of a block, then an explicit inner block can be used, as in the idiom `do return end`, because now **return** is the last statement in its (inner) block.

### for statement

```
stat ::= for Name '=' exp ',' exp [',' exp] do block end
```

```
stat ::= for namelist in pairs(x) do block end
namelist ::= Name {',' Name}
x is a table
```

when iterating over a table, the order of elements is not guaranteed. 

The following code:

```lua
a = {1, 2, 3}
for k in pairs(a) do 
    print(k)
end
```

may print `2 3 1` or something else.

### function calls as statements

```
stat ::= functioncall
```

### local declarations

Local variables can be declared anywhere inside a block. The declaration can include an initial assignment:

```
stat ::= local namelist ['=' explist]
```

If present, an initial assignment has the same semantics of a multiple assignment. Otherwise, all variables are initialized with **nil**.

## Expressions

### arithmetic operators

- `+` addition 

- `-` subtraction 

- `*` multiplication 

- `/` float division

- `//` floor division

- `%` modulo

- `^` exponation

- `-` unary minus

when performing arithmetic operations, if the operand is a string that can be converted to numbers, the operation is still valid.

### relational operators

- `==` equality

- `~=` inequality

- `>` greater than

- `<` less than

- `>=` greater or equal

- `<=` less or equal

these operators always results in **true** or **false**

Equality first compares the type of its operands. If the types are different, the result is **false**. Otherwise, the value of the operands are compared. Tables and functions are compared by reference: two objects are equal if they refer to the same object, otherwise the result is false.

`~=` is the negation of `==` 

comparison `a > b` is translated to `b < a` and `a >= b` is translated to `b <= a`. 

### logical operators

the operators are **and**, **or** and **not**. all logical operators consider both **false** and **nil** as false and anything else as true. 

**and** and **or** follows the short-circuit evaluation rule. So the following code should not throw an error:

```lua
a = 1
b = nil
if a == 1 or b[1] == 0 then  -- this should not throw "attempt to index nil value" error
    print("yes") 
end
```

### concatenation

If both operands are strings or numbers, then they are converted to strings

```lua
a = "hello, "
b = "world"
print(a..b) -- "hello, world"
c = "2023"
print(a..c) -- "hello, 2023"
```

### the length operator

`#` can be applied on *string* and *table*. 

for string, it will get its number of bytes. 

for table, it will get its number of elements. 

### precedence

Operator precedence in Lua follows the table below, from lower to higher priority:

     or
     and
     <     >     <=    >=    ~=    ==
     ..
     +     -
     *     /     //    %
     unary operators (not   #     -)
     ^

parentheses `()` can be used to change the precedences of an expression. 

### table constructors

Table constructors are expressions that create tables. Every time a constructor is evaluated, a new table is created. A constructor can be used to create an empty table or to create a table and initialize some of its fields. The general syntax for constructors is

    tableconstructor ::= '{' [fieldlist] '}'
    fieldlist ::= field {fieldsep field} [fieldsep]
    field ::= Name '=' exp | exp
    fieldsep ::= ',' | ';'

The order of assignment is from left to right. This only matters when there are repeated keys. 

If the last field in the list has the form `exp` and the expression is a functioncall, then all values returned by this expression enter the list consecutively.

```lua
function foo()
    return 1, 2, 3
end

a = {100, 200, foo()}
-- now table 'a' contains {100, 200, 1, 2, 3}
print(#a) -- 5
a = {100, 200, b=foo()}
-- now table 'a' contains {100, 200, b=1}
print(#a) -- 3
```

### function calls

A function call in Lua has the following syntax:

    functioncall ::= prefixexp args

Arguments have the following syntax:

    args ::= ‘**(**’ [explist] ‘**)**’
    args ::= tableconstructor
    args ::= LiteralString

All argument expressions are evaluated before the call. A call of the form `f{fields}` is syntactic sugar for `f({fields})`; that is, the argument list is a single new table. A call of the form `f'string'` (or `f"string"` or `f[[string]]`) is syntactic sugar for `f('string')`; that is, the argument list is a single literal string.

```lua
function foo(s)
    print(s)
end
foo"hello, world" -- hello, world
foo[[hello, 2023] -- hello, 2023
foo'hello, pl' -- hello, pl

function goo(t)
    print(t[1])
    print(t[2])
end
goo{3,4} -- 3, 4
```

### function definitions

The syntax for function definition is

    functiondef ::= function funcbody
    funcbody ::= '(' [parlist] ')' block end

The following syntactic sugar simplifies function definitions:

    stat ::= [local] function Name funcbody

The statement 

```
function f() body end
```

is equal to 

```
f = function () body end
```

A function definition is an executable expression, when Lua executes the function definition, the function will take a snapshot of the stack and stores that information into its 'closure' field. In the body of the function, the closure will be accessed first when its trying to reference a variable. 

For example, 

```lua
function foo()
    local a = 100
    function goo()
        return a
    end

    return goo
end

a = 200
print(foo()()) -- 100
print(a)   -- 200
```

when `goo` is instantiated, `a=100` is 'remembered' by `goo`. When we call `goo`, we can still get `100` as return value.

When a function is called, the list of arguments is adjusted to the length of the list of parameters

```lua
function foo()
    return 1, 2, 3
end

function add(a, b, c)
    return a + b + c
end

print(add(foo())) -- 6
```

## Visibility Rules

Lua is a lexically scoped language. The scope of a local variable begins at the first statement after its declaration and lasts until the last non-void statement of the innermost block that includes the declaration. Consider the following example:

     x = 10                -- global variable
     do                    -- new block
       local x = x         -- new 'x', with value 10
       print(x)            --> 10
       x = x+1
       do                  -- another block
         local x = x+1     -- another 'x'
         print(x)          --> 12
       end
       print(x)            --> 11
     end
     print(x)              --> 10  (the global one)
