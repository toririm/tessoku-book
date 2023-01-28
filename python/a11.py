# A11 Binary Search 1

n, x = map(int, input().split())
a = list(map(int, input().split()))

def search(y: int) -> int: # 0-based
    l, r = 0, n
    while (l <= r):
        m = (l + r) // 2
        if a[m] < y:
            l = m
        if a[m] == y:
            return m
        if a[m] > y:
            r = m
    return -1

print(search(x) + 1)