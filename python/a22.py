# A22 Sugoroku
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38517660

n = int(input())
a = list(map(int, input().split())) # 0-based
b = list(map(int, input().split()))

dp = [- 10 ** 9] * (n + 1) # 1-based
dp[1] = 0

for i in range(n - 1):
    dp[a[i]] = max(dp[a[i]], dp[i + 1] + 100)
    dp[b[i]] = max(dp[b[i]], dp[i + 1] + 150)

print(dp[n])