# A09 Winter in ALGO Kingdom
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38345053

h, w, n = map(int, input().split())
x = [[0] * (w + 1) for i in range(h + 1)]

for t in range(n):
    a, b, c, d = map(int, input().split())
    a -= 1
    b -= 1
    x[c][d] += 1
    x[a][d] -= 1
    x[c][b] -= 1
    x[a][b] += 1

for i in range(h + 1):
    for j in range(w):
        x[i][j + 1] += x[i][j]

for i in range(h):
    for j in range(w + 1):
        x[i + 1][j] += x[i][j]

for i in range(h):
    print(' '.join([str(x[i][j]) for j in range(w)]))