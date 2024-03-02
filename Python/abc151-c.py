tmp = input().split()
N = int(tmp[0])
M = int(tmp[1])

penalty_dic = {}
ac_set = set()

for i in range(0, M):
    tmp = input().split()
    p = int(tmp[0])
    s = tmp[1]
    if p not in ac_set and s == 'WA':
        penalty_dic[p] = penalty_dic.get(p, 0) + 1
    if s == 'AC':
        ac_set.add(p)

num_ac = len(ac_set)
num_penalty = sum([v for k, v in penalty_dic.items() if k in ac_set])

print(num_ac, num_penalty)

