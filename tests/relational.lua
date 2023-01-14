-- test relational operator
print(1==2) -- false
print(1~=2) -- true
print(1 > 2) -- false
print(2 > 2) -- false
print(2 >= 2) -- true
print(2 == 2) -- true
print(1 < 2) -- true
print(1 <= 2) -- true
print(2 <= 2) -- true
print(2 < 2) -- false
print(0.1 + 0.2 == 0.3) -- false

print("--------------------")

a = {1,2,3}
b = {1,2,3}
print(a == b)  -- false, they are not the same object
print(a == a)  -- true, it is the same object
print(123 == "123") -- false, the types are different