# A04 Binary Representation 1
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38326161

n = int(input())
ans = []

for i in range(10)[::-1]:
    ans.append(str(int(bool(n & (1 << i)))))
print(''.join(ans))