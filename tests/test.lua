function foo()
    a = {1,2,3}
    return a
end

for k, v in pairs(foo()) do
    print(k, v)
end