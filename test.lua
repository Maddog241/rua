a = 1
b = 2
print(a)
print(b)
print(c)
a = 3
b = 4
print(a)
print(b)

for i=0,10,1 do 
    print(i)
end

function pow5(x)
    return (x * x) * (x * x) * x
end

print(pow5(1))
print(pow5(2))

-- this is a line comment
-- now we will check the strings

str_a = "string a"
str_b = "string b"
str_c = str_a .. str_b
print(str_a, str_b, stc_c)

--[[
    this is a cross line comment
--]]

print("end of the program")