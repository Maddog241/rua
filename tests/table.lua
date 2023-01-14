function foo()
    return 1, 2, 3
end

a = {100, 200, b=foo()}
print("length of a:", #a)
print("a.b:", a.b)
print("a['b']", a['b'])

b = {9, 8; 7, 6;}
print("length of b:", #b)
print("b: ", b[1], b[2], b[3], b[4])


--- delete a value from table
print("delete a value from table")
b[1] = nil
print("length of b:", #b)
print("b: ", b[1], b[2], b[3], b[4])