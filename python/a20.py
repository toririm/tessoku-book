# A20 LCS
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38504375

s = input()
t = input()

sl = len(s)
tl = len(t)
dp = [[0] * (tl + 1) for i in range(sl + 1)]

for i in range(sl + 1):
    for j in range(tl + 1):
        if i < sl and j < tl and s[i] == t[j]:
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + 1)
        if i < sl:
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])
        if j < tl:
            dp[i][j + 1] = max(dp[i][j + 1], dp[i][j])

print(dp[sl][tl])