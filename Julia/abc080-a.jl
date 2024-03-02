n, a, b = parse.(Int, readline() |> split)
ans = min(n * a, b)
println(ans)
