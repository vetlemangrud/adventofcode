zeros = 0
with open("input", "r") as file:
    dial = 50
    prev = 50
    for line in file.readlines():
        print(line)
        dir = line[0]
        amount = int(line[1:])
        if (dir == "L"):
            dial -= amount 
        else:
            dial += amount
        if (dial >= 100):
            zeros += dial // 100
        if (dial <= 0):
            zeros += abs(dial) // 100
            if(prev != 0):
                zeros += 1
        dial = (dial + 100) % 100
        prev = dial
        print(dial)
        print(zeros)
print(zeros)
