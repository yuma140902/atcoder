s = io.read()
l = s:len()
if s:sub(l, l) == "T" then
  print("YES")
else
  print("NO")
end
