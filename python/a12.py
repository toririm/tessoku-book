# A12 Printer
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38444341

n, k = map(int, input().split())
a = list(map(int, input().split()))

def is_fulfilled(t: int) -> bool:
    return sum(t // aa for aa in a) >= k

def search(y: int) -> int:
    l, r = 1, 10 ** 9
    while l < r:
        m = (l + r) // 2
        if is_fulfilled(m):
            r = m
        else:
            l = m + 1
    return l # == r

print(search(k))