x = 10                -- global variable
do                    -- new block
local x = x         -- new 'x', with value 10
print(x)            --> 10
x = x+1
do                  -- another block
    local x = x+1     -- another 'x'
    print(x)          --> 12
end
print(x)            --> 11
end
print(x)              --> 10  (the global one)

print("-----------")
a = {}
local x = 20
for i=1,10 do
    local y = i
    a[i] = function () y=y+1; return x+y end
end

for i=1,10 do 
    print(a[i]())
end