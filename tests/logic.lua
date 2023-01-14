a = 1 
b = nil
if a == 1 or b[1] == 0 then  -- this should not throw "attempt to index nil value" error
    print("no error")
end

-- 

-- a = 2

if nil then
    print("this should not be printed")
elseif false then
    print("this should not be printed, either")
elseif a == 1 then
    print("hello, a==1")
else 
    print("a ~= 1")
end