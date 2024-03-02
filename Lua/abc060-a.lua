a, b, c = io.read():match("(%w+) (%w+) (%w+)")
if a:sub(a:len(), a:len()) == b:sub(1, 1) and b:sub(b:len(), b:len()) == c:sub(1, 1) then
  print("YES")
else
  print("NO")
end
