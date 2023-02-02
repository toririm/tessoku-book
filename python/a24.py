# A24 LIS
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38543938

from bisect import bisect_left

n = int(input())
a = list(map(int, input().split()))

dp = [0] * n
l = [10 ** 6] * n

for i in range(n):
    pos = bisect_left(l, a[i])
    dp[i] = pos + 1
    l[pos] = a[i]

print(max(dp[i] for i in range(n) if dp[i] != 10 ** 6))