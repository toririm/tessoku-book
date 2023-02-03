# A29 Power
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38574873

MODULO = 10 ** 9 + 7

def power(x: int, y: int) -> int:
    ret = 1
    tmp = x
    for i in range(30):
        if y & (1 << i):
            ret *= tmp
            ret %= MODULO
        tmp **= 2
        tmp %= MODULO
    return ret


a, b = map(int, input().split())
print(power(a, b))