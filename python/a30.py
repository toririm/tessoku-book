# A30 Combination
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38582866

MODULO = 10 ** 9 + 7

def factorial(n: int) -> int:
    ret = 1
    for i in range(1, n + 1):
        ret *= i
        ret %= MODULO
    return ret

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

def inverse(b: int) -> int:
    b %= MODULO
    return power(b, MODULO - 2)

n, r = map(int, input().split())

print((factorial(n) * inverse(factorial(r) * factorial(n - r))) % MODULO)