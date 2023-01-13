function foo()
    return 1, 2, 3
end

a, b, c, d = 0, foo(), 2
print(a, b, c, d)