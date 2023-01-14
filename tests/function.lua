function foo()
    return 1, 2, 3
end

function add(a, b, c)
    return a + b + c
end

print(add(foo())) -- 6

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