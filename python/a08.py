# A08 Two Dimensional Sum
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38344227

h, w = map(int, input().split())
x = [[0] * (w + 1)] + [[]] * h

for i in range(h):
    x[i + 1] = [0] + list(map(int, input().split()))

for i in range(h + 1):
    for j in range(w):
        x[i][j + 1] += x[i][j]

for i in range(h):
    for j in range(w + 1):
        x[i + 1][j] += x[i][j]

q = int(input())

for qq in range(q):
    a, b, c, d = map(int, input().split())
    a -= 1
    b -= 1
    print(x[c][d] - x[a][d] - x[c][b] + x[a][b])