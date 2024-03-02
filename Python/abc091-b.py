N = int(input())
s = []
for i in range(0, N):
    s.append(input())
M = int(input())
t = []
for i in range(0, M):
    t.append(input())

s_dic = {}
t_dic = {}

for si in s:
    s_dic[si] = s_dic.get(si, 0) + 1
for ti in t:
    t_dic[ti] = t_dic.get(ti, 0) + 1

points = []
for si, count in s_dic.items():
    p = count - t_dic.get(si, 0)
    points.append(p)

print(max(max(points), 0))
