# A06 How Many Guests?
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38344037

n, q = map(int, input().split())
a = [0] + list(map(int, input().split()))

for i in range(n):
    a[i + 1] += a[i]

for j in range(q):
    l, r = map(int, input().split())
    print(a[r] - a[l - 1])