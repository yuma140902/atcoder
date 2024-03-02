a, b, c = parse.(Int, readline() |> split)
if a == b
  println(c)
elseif b == c
  println(a)
else
  println(b)
end
