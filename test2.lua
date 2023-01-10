a = "global"
do 
    function showA()
        print(a)
    end
    
    showA()
    a = "local"
    showA()
end