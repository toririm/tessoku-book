# A26 Prime Check
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38566244

q = int(input())

MAX = 3 * 10 ** 5
is_prime = [True] * (MAX + 1) # 1-based
is_prime[0:1] = [False, False]

for n in range(MAX + 1):
    if is_prime[n]:
        not_prime = n + n
        while not_prime <= MAX:
            is_prime[not_prime] = False
            not_prime += n

for _q in range(q):
    x = int(input())
    if is_prime[x]:
        print('Yes')
    else:
        print('No')