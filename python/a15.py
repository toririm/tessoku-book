# A15 Compression
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38499627

from bisect import bisect_left

n = int(input())
a = list(map(int, input().split()))

c = list(set(a))
c.sort()
b = []
for aa in a:
    b.append(str(bisect_left(c, aa) + 1))

print(' '.join(b))