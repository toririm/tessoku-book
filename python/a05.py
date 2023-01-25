# A05 Three Cards
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38326350

n, k = map(int, input().split())
ans = 0

for r in range(1, n + 1):
    for b in range(1, n + 1):
        w = k - r - b
        if 1 <= w <= n:
            ans += 1

print(ans)