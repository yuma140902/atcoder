tmp = input().split()
x1 = int(tmp[0])
y1 = int(tmp[1])
x2 = int(tmp[2])
y2 = int(tmp[3])

x2 -= x1
y2 -= y1

x4 = -y2
y4 = x2
x3 = x2 + x4
y3 = y2 + y4

x4 += x1
y4 += y1
x3 += x1
y3 += y1

print(x3, y3, x4, y4)
