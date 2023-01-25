# A02 Liner Search
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38325625

N, X = map(int, input().split())
A = list(map(int, input().split()))

if any(Ai == X for Ai in A):
    print('Yes')
else:
    print('No')