alpha = io.read()
n = io.read("*n")
l = alpha:len()

m = (n - 1) % l + 1
k = (n - m) / l + 1
io.write(alpha:sub(k, k))
io.write(alpha:sub(m, m))
print()
