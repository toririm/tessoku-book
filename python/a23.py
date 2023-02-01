# A23 All Free
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38518802

n, m = map(int, input().split())
a = [list(map(int, input().split())) for i in range(m)]

dp = [[1000] * (2 ** n) for i in range(m + 1)] # bit DP
dp[0][0] = 0

for bit in range(2 ** n):
    for i in range(m):
        new_bit = bit
        for j in range(n):
            new_bit = new_bit | (a[i][j] << j)
        dp[i + 1][bit] = min(dp[i + 1][bit], dp[i][bit])
        dp[i + 1][new_bit] = min(dp[i + 1][new_bit], dp[i][bit] + 1)

ans = dp[m][2 ** n - 1]
if ans == 1000:
    print(-1)
else:
    print(ans)