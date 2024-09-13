-- This is a comment

-- Variables and assignments
local x = 10
local y = 20.5
local z = "Hello, world!"
local multiline_string = [[
	 This is a
	 multiline string
]]

-- Functions
function greet(name)
	 print("Hello, " .. name .. "!")
end

local function add(a, b)
	 return a + b
end

-- Control structures
if x > 5 then
	 print("x is greater than 5")
elseif x < 5 then
	 print("x is less than 5")
else
	 print("x is equal to 5")
end

for i = 1, 5 do
	 print(i)
end

local count = 0
while count < 3 do
	 print("Count: " .. count)
	 count = count + 1
end

-- Tables
local fruits = {"apple", "banana", "orange"}
local person = {
	 name = "John",
	 age = 30,
	 isStudent = false
}

-- Metatables
local mt = {
	 __add = function(a, b)
		  return { value = a.value + b.value }
	 end
}

local obj1 = { value = 10 }
local obj2 = { value = 20 }
setmetatable(obj1, mt)
setmetatable(obj2, mt)

local result = obj1 + obj2
print(result.value)  -- Output: 30

-- Coroutines
local co = coroutine.create(function()
	 for i = 1, 3 do
		  print("Coroutine: " .. i)
		  coroutine.yield()
	 end
end)

coroutine.resume(co)
coroutine.resume(co)
coroutine.resume(co)

-- Error handling
local success, err = pcall(function()
	 error("This is a test error")
end)

if not success then
	 print("Caught error: " .. err)
end

-- Modules
local mymodule = {}

function mymodule.sayHello()
	 print("Hello from module!")
end

return mymodule