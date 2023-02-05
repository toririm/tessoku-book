# A31 Divisors
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38638837

def divisor(max: int, a: int) -> int:
    return max // a

n = int(input())

print(divisor(n, 3) + divisor(n, 5) - divisor(n, 15))