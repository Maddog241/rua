a = 100
do 
    local a = 200
    print(a) -- 200
    do 
        local a = 300
        print(a) -- 300
        a = a + 100
        print(a) -- 400
    end
    print(a) -- 200
end
print(a) -- 100