# A18 Subset Sum
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38502468

n, s = map(int, input().split())
a = list(map(int, input().split()))

dp = [[False] * (s + 1) for i in range(n + 1)]
dp[0][0] = True

for i in range(n):
    for j in range(s + 1):
        if j + a[i] <= s:
            dp[i + 1][j + a[i]] += dp[i][j]
        dp[i + 1][j] += dp[i][j]

if dp[n][s]:
    print('Yes')
else:
    print('No')