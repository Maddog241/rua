a = 'alo\n123"'
b = [[alo
123"]]
c = [[
alo
123"]]

--[[
    the following should all evaluate to true
]]
print(a == b) -- true
print(b == c) -- true
print(a == c) -- true


-- test the escape sequences
print("programming\nlanguage") 
print("programming\tlanguage") 
print("programming\\language") 
print("programming\'language")
print("programming\"language")