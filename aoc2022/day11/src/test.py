s = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
"""

lines = s.splitlines()[2:]

print(lines[0][18:])  # items
print(lines[1][23])  # operator
print(lines[1][25:])  # val
print(lines[2][21:])  # divisor
print(lines[3][29:])  # true idx
print(lines[4][30:])  # false idx
