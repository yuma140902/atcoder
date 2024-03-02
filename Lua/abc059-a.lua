s, t, u = io.read():match("(%w+) (%w+) (%w+)")
a = s:sub(1, 1) .. t:sub(1, 1) .. u:sub(1, 1)
a = a:upper()
print(a)
