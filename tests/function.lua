function add(a, b)
    return a + b
end

function foo()
    return 1, 2
end

print(add(foo()))

function foo(s)
    print(s)
end

foo"hello, world" -- hello, world
foo[[hello, 2023]] -- hello, 2023
foo'hello, pl' -- hello, pl

function goo(t)
    print(t[1])
    print(t[2])
end
goo{3,4} -- 3, 4