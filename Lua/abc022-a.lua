n, s, t = io.read("*n", "*n", "*n")
c = 0
w = 0
for i = 1, n do
  w = w + io.read("*n")
  if s <= w and w <= t then
    c = c + 1
  end
end
print(c)
