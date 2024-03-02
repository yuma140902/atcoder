a, b = io.read("*n", "*n")
if a == 1 then a = 14 end
if b == 1 then b = 14 end
if a > b then
  print("Alice")
elseif a < b then
  print("Bob")
else
  print("Draw")
end
