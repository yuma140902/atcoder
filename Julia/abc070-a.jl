n = parse(Int, readline())
c = n % 10
a = n ÷ 100
if a == c
  println("Yes")
else
  println("No")
end
