# A23 All Free
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38518895

n, m = map(int, input().split())
a = [list(map(int, input().split())) for i in range(m)]

bits = 2 ** n
dp = [[1000] * bits for i in range(m + 1)] # bit DP
dp[0][0] = 0

for bit in range(bits):
    for i in range(m):
        new_bit = bit
        for j in range(n):
            new_bit = new_bit | (a[i][j] << j)
        dp[i + 1][bit] = min(dp[i + 1][bit], dp[i][bit])
        dp[i + 1][new_bit] = min(dp[i + 1][new_bit], dp[i][bit] + 1)

ans = dp[m][bits - 1]
if ans == 1000:
    print(-1)
else:
    print(ans)