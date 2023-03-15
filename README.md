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


## Examples

```lua
a, b, c = 1, 2, 3, 4
print(a, b, c) -- 1, 2, 3

d, e, f = 100, 200
print(d, e, f) -- 100, 200, nil

function foo() 
    return 1, 2, 3
end

g, h, i, j, k = 0, foo()
print(g, h, i, j, k) -- 0, 1, 2, 3, nil

--[[
    The assignment statement first evaluates all 
    its expressions and only then the assignments 
    are performed.
]]

a = {1, 2, 3, 4}
i = 3
i, a[i] = i+1, 20
print(i, a[3], a[4])  -- 4, 20, 4

x, y, z = 10, 20, 30
x, y, z = y, z, x
print(x, y, z) -- 20, 30, 10
```

```lua
-- return a closure
function wrapper(a, b)
    function subtract()
        return a - b
    end

    return subtract
end

foo = wrapper(1,2)
print(foo()) --  -1
```

```lua
a = {100, 200, b=foo()}
print("length of a:", #a)
print("a.b:", a.b)
print("a['b']", a['b'])

b = {9, 8; 7, 6;}
print("length of b:", #b)
print("b: ", b[1], b[2], b[3], b[4])
```