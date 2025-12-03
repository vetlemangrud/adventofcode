with open("input", "r") as file:
    line = file.readlines()[0]


total = 0
for r in line.split(","):
    (start,end) = r.split("-")
    for i in range(int(start),int(end)+1):
        string = str(i)
        for rep_len in range(1, len(string) // 2 + 1):
            if len(string) % rep_len != 0:
                continue
            rep_num = string[:rep_len]
            found = True
            for start_i in range(0, len(string), rep_len):
                if string[start_i:start_i+rep_len] != rep_num:
                    found = False
                    break

            if found:
                print(string)
                total += int(string)
                break

print(total)

# not 79244580806
