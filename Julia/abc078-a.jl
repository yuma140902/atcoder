X, Y = parse.(Int, readline() |> split, base=16)
if X < Y
  println("<")
elseif X > Y
  println(">")
else
  println("=")
end
