a = {}
local x = 20
for i=1,10 do
    local y = i
    a[i] = function () y=y+1; return x+y end
end

for i=1,10 do 
    print(a[i]())
end