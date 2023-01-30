# A14 Four Boxes
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38476976

from bisect import bisect_left

n, k = map(int, input().split())
abcd = [list(map(int, input().split())) for i in range(4)]

p = []
for i in range(n):
    for j in range(n):
        p.append(abcd[0][i] + abcd[1][j])

q = []
for i in range(n):
    for j in range(n):
        q.append(abcd[2][i] + abcd[3][j])

q.sort()
for pp in p:
    index = bisect_left(q, k - pp)
    if index < n * n and q[index] == k - pp:
        print('Yes')
        quit()
print('No')