# A11 Binary Search 1
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38363066

n, x = map(int, input().split())
a = list(map(int, input().split()))

def search(y: int) -> int: # 0-based
    l, r = 0, n - 1
    while l <= r:
        m = (l + r) // 2
        if a[m] < y:
            l = m + 1
        if a[m] == y:
            return m
        if a[m] > y:
            r = m - 1
    return -1

print(search(x) + 1) # 1-based