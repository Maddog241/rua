--- 
function foo()
    local a = 100
    function goo()
        return a
    end

    return goo
end

a = 200
print(foo()()) -- 100
print(a)   -- 200