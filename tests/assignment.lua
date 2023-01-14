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