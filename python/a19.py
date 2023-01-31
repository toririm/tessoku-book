# A19 Knapsack 1
# submission(PyPy3): https://atcoder.jp/contests/tessoku-book/submissions/38503186

n, w = map(int, input().split())
wv = [tuple(map(int, input().split())) for i in range(n)]

dp = [[0] * (w + 1) for i in range(n + 1)]

for i in range(n):
    for j in range(w + 1):
        if j + wv[i][0] <= w:
            dp[i + 1][j + wv[i][0]] = max(dp[i + 1][j + wv[i][0]], dp[i][j] + wv[i][1])
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])

print(dp[n][w])