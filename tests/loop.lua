--- numeric for 
---- 1, 2, ..., 5
for i=1,10 do 
    print(i)
    if i >= 5 then
        break
    end
end

t = {-1, -2, -3, -4, -5, -6}
--- generic for
--[[ note: the order of (k,v) pair is not guaranteed, this 
is explained in doc.md
]]
for k, v in pairs(t) do 
    print(k, v)
end

--- while loop
--- 10, 9, ...., 4
a = 10
while a > 0 do
    print(a)
    a = a - 1
    if a < 4 then 
        break
    end
end