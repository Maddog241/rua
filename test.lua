--- 
do 
    local a = 'hello'
    function fun()
        print(a)
    end
    fun()
end
fun()
---
a = "hello"
do 
    function fun() 
        print(a)
    end

    fun()
    a = "world"
    fun()
end