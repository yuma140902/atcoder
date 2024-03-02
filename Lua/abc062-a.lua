x, y = io.read("*n", "*n")
a = { 1, 3, 5, 7, 8, 10, 12 }
b = { 4, 6, 9, 11 }
c = { 2 }
function contains(a, v)
  for i = 1, #a do
    if a[i] == v then
      return true
    end
  end
  return false
end

if (contains(a, x) and contains(a, y))
    or (contains(b, x) and contains(b, y))
    or (contains(c, x) and contains(c, y)) then
  print("Yes")
else
  print("No")
end
