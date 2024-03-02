r, g, b = io.read("*n", "*n", "*n")
a = r * 100 + g * 10 + b
if a % 4 == 0 then
  print "YES"
else
  print "NO"
end
