# A02 Liner Search
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38326123

n, x = map(int, input().split())
a = list(map(int, input().split()))

if any(aa == x for aa in a):
    print('Yes')
else:
    print('No')