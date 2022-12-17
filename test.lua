do 
    print(a)
end

for i=1, 10, 1 do
    print(i)
    a, b, c = 1, 2, 3
    if i > 3 then 
        break
    end
end

i = 1
while i < 10 do 
    print(i)
    myfunc = function(a, b) 
        return a + b
    end
    if i < 4 then 
        print("i < 4")
    elseif i < 7 then 
        print("4 <= i < 7")
    else 
        print("i >= 7")
    end

    print("3+i", myfunc(3, i))
end

for i in 1, 2, 3 do 
    print(i)
end

function add(a, b) 
    return a + b
end

mytable = {a=1, c=b; 1, 2, 3}