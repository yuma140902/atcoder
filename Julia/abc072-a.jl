x, t = parse.(Int, readline()|>split)
ans = max(x - t, 0)
println(ans)
