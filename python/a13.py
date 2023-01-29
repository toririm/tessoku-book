# A13 Close Pairs
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38446807

n, k = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
r = 1
for l in range(n - 1):
    while r < n and a[r] - a[l] <= k:
        r += 1
    r -= 1
    ans += r - l

print(ans)