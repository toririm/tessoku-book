# A27 Calculate GCD
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38566598

def gcd(x: int, y: int) -> int:
    if x % y == 0:
        return y
    return gcd(y, x % y)

a, b = map(int, input().split())

print(gcd(a, b))