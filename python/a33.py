# A33 Game 2
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38740310

n = int(input())
a = list(map(int, input().split()))

xor_sum = a[0]
for i in range(n - 1):
    xor_sum ^= a[i + 1]

if xor_sum == 0:
    print('Second')
else:
    print('First')