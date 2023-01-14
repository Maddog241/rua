a = 'alo\n123"'
b = [[alo
123"]]
c = [[
alo
123"]]

--[[
    the following should all evaluate to true
]]
print(a == b)
print(b == c)
print(a == c)