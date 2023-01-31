# A21 Block Game
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38509670

n = int(input())
pa = [(0, 0)] + [tuple(map(int, input().split())) for i in range(n)] # 1-based

dp = [[0] * (n + 1) for i in range(n + 1)]

for len in reversed(range(1, (n - 1) + 1)): #len = r - l
    for l in range(1, (n - len) + 1):
        r = l + len
        rpoint = 0
        if l <= pa[r][0] <= r:
            rpoint = pa[r][1]
        lpoint = 0
        if l <= pa[l][0] <= r:
            lpoint = pa[l][1]
        dp[l][r - 1] = max(dp[l][r - 1], dp[l][r] + rpoint)
        dp[l + 1][r] = max(dp[l + 1][r], dp[l][r] + lpoint)

ans = 0
for i in range(1, n + 1):
    ans = max(ans, dp[i][i])

print(ans)