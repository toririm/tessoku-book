# A25 Number of Routes
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38565471

h, w = map(int, input().split())
c = [list(input()) for i in range(h)] # 0-based

dp = [[0] * (w + 1) for i in range(h + 1)] # 1-based + 余白

# 貰うdpのほうが書きやすい
for i in range(h):
    for j in range(w):
        if i == 0 and j == 0:
            dp[i + 1][j + 1] = 1
            continue
        if c[i][j] == '.':
            dp[i + 1][j + 1] = dp[i][j + 1] + dp[i + 1][j]

print(dp[h][w])
