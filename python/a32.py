# A32 Game 1
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38677956

n, a, b = map(int, input().split())

win = [False] * (n + 1)

for i in range(n + 1):
    if win[i] == False:
        if i + a <= n:
            win[i + a] = True
        if i + b <= n:
            win[i + b] = True

if win[n]:
    print('First')
else:
    print('Second')