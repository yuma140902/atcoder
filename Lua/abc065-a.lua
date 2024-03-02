x, a, b = io.read("*n", "*n", "*n")
y = b - a
if y <= 0 then
  print "delicious"
elseif y <= x then
  print "safe"
else
  print "dangerous"
end
