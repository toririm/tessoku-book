# A28 Blackboard
# submission: https://atcoder.jp/contests/tessoku-book/submissions/38567348

MODULO = 1_00_00

class Modulo:
    def __init__(self, value: int, modulo: int):
        self.value = value
        self.modulo = modulo
    def __str__(self) -> str:
        return str(self.value)
    def __iadd__(self, other):
        if not type(other) is Modulo or self.modulo != other.modulo:
            raise TypeError()
        self.value = (self.value + other.value) % self.modulo
        return self
    def __isub__(self, other):
        if not type(other) is Modulo or self.modulo != other.modulo:
            raise TypeError()
        self.value = (self.value - other.value) % self.modulo
        return self
    def __imul__(self, other):
        if not type(other) is Modulo or self.modulo != other.modulo:
            raise TypeError()
        self.value = self.value * other.value % self.modulo
        return self

n = int(input())

b = Modulo(0, MODULO)

for i in range(n):
    t, a = input().split()
    a = Modulo(int(a), MODULO)
    if t == '+':
        b += a
    if t == '-':
        b -= a
    if t == '*':
        b *= a
    print(b)
