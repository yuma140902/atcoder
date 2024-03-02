x, a, b = parse.(Int, readline()|>split)
if abs(x - a) < abs(x - b)
  println("A")
else
  println("B")
end
