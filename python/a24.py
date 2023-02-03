# A24 LIS
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38565011

from bisect import bisect_left

n = int(input())
a = list(map(int, input().split()))

dp = [10 ** 6] * (n + 1) # dp[j]: 長さjのLISの最後の値として最小のもの
dp[0] = 0 # A[i] > 0 なので dp[0] = 0 とすると上手くいく
ans = 0

for i in range(n):
    j = bisect_left(dp, a[i]) # j: sortを崩さずa[i]を挿入できるindex <--> a[i]は長さjのLISの最後の値で最小のもの
    ans = max(ans, j)
    dp[j] = a[i]

print(ans)