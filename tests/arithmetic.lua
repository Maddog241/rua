a = 1
b = nil
if a == 1 or b[1] == 0 then  -- this should not throw "attempt to index nil value" error
    print("yes") 
end

a = "hello, "
b = "world"
print(a..b) -- hello, world
c = 2023
print(a..c) -- hello, 2023