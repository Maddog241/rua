-- addition
print(3975 + 1935)
print(3975 + "1935")
print("3975" + "1935")
print("3975" + 1935)

-- subtraction
print(3975 - 1935)
print(3975 - "1935")
print("3975" - "1935")
print("3975" - 1935)

-- multiplication
print(3975 * 1935)
print(3975 * "1935")
print("3975" * "1935")
print("3975" * 1935)

-- float division
print(3975 / 1935)
print(3975 / "1935")
print("3975" / "1935")
print("3975" / 1935)

-- floor division
print(3975 // 1935)
print(3975 // "1935")
print("3975" // "1935")
print("3975" // 1935)

-- modulo
print(3975 % 1935)
print(3975 % "1935")
print("3975" % "1935")
print("3975" % 1935)

-- exponation
print(3975 ^ 2.5)
print(3975 ^ "2.5")
print("3975" ^ "2.5")
print("3975" ^ 2.5)

-- unary minus
print(-"5216.2145")
print(-5216.2145)

----------

a = 1
b = nil
if a == 1 or b[1] == 0 then  -- this should not throw "attempt to index nil value" error
    print("yes") 
end

a = "hello, "
b = "world"
print(a..b) -- hello, world
c = 2023
print(a..c) -- hello, 2023
