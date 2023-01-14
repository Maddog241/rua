a = "hello, world"
print(#a) -- 12
b = {1,2,3, hello=100}
print(#b) -- 4
b[1] = nil
print(#b) -- 3
b["hello"] = nil
print(#b) -- 2