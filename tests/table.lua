function foo()
    return 1, 2, 3
end

a = {100, 200, b=foo()}
print(#a)
print(a.b)