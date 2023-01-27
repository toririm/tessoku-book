# A10 Resort Hotel
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38351106

n = int(input())
a = list(map(int, input().split()))

max_left = a.copy()
max_right = a.copy()

for i in range(n - 1):
    max_left[i + 1] = max(max_left[i + 1], max_left[i])
for i in range(1, n)[::-1]:
    max_right[i - 1] = max(max_right[i - 1], max_right[i])

d = int(input())

for dd in range(d):
    l, r = map(int, input().split())
    l -= 1
    r -= 1
    print(max(max_left[l - 1], max_right[r + 1]))