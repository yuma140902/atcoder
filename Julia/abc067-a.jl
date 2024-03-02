a, b = parse.(Int, split(readline()))
if a % 3 == 0 || b % 3 == 0 || (a+b) % 3 == 0
  println("Possible")
else
  println("Impossible")
end
