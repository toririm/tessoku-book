# A16 Dungeon 1
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38500503

n = int(input())
a = list(map(int, input().split()))
b = list(map(int, input().split()))

dp = [10 ** 9] * n
dp[0] = 0

for i in range(n - 1):
    dp[i + 1] = min(dp[i + 1], dp[i] + a[i])
    if i < n - 2:
        dp[i + 2] = min(dp[i + 2], dp[i] + b[i])

print(dp[n - 1])