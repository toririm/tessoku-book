# A07 Event Attendance
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38344110

d = int(input())
n = int(input())

a = [0] * (d + 1)

for i in range(n):
    l, r = map(int, input().split())
    a[l - 1] += 1
    a[r] -= 1

for j in range(d):
    a[j + 1] += a[j]
    print(a[j])