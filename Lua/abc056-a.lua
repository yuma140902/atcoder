function f(c)
  if c == "H" then
    return 1
  else
    return -1
  end
end

a, b = io.read("*l"):match("(%w) (%w)")
a = f(a)
b = f(b)
c = a * b
if c == 1 then
  print("H")
else
  print("D")
end
