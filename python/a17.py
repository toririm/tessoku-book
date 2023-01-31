# A17 Dungeon 2

n = int(input())
a = list(map(int, input().split()))
b = list(map(int, input().split()))

dp = [10 ** 8] * n
dp[0] = 0

for i in range(n - 1):
    dp[i + 1] = min(dp[i + 1], dp[i] + a[i])
    if i < n - 2:
        dp[i + 2] = min(dp[i + 2], dp[i] + b[i])

p = []
p.append(n)

i = n - 1
while i > 0:
    if dp[i] == dp[i - 1] + a[i - 1]:
        p.append(i)
        i = i - 1
    if i > 1 and dp[i] == dp[i - 2] + b[i - 2]:
        p.append(i - 1)
        i = i - 2

p.reverse()
print(len(p))
print((' ').join(str(i) for i in p))