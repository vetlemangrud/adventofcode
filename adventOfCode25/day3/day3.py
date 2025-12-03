def maxjolts(batteries: list[int], amount: int)->int:
    if (amount == 1):
        first = max(batteries)
    else:
        first = max(batteries[:-(amount-1)])
    first_i = batteries.index(first)
    rest = 0
    if (amount > 1):
        rest = maxjolts(batteries[first_i+1:], amount-1)
    return first * 10 ** (amount -1) + rest




with open("input", "r") as file:
    lines = file.readlines()

total = 0
for line in lines:
    line = line.strip()
    print(line, end=": ")
    batteries = [int(c) for c in line]
    max_jolt = maxjolts(batteries,12)
    print(max_jolt)
    total += max_jolt

print(total)
