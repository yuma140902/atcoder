a, b, c = parse.(Int, split(readline()))
min = minimum([a+b, b+c, c+a])
println(min)
