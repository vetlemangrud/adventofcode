import re

with open("input", "r") as file:
    input = file.read()
    not_part_regex = re.compile("(?s)(?<=\.{5}.{138}\.)\d{3}(?=\..{138}\.{5})|(?<=\.{4}.{139}\.)\d{2}(?=\..{139}\.{4})|(?<=\.{3}.{140}\.)\d{1}(?=\..{140}\.{3})")
    not_part_res = not_part_regex.findall(input)
    total_not_part = 0
    for num in not_part_res:
        total_not_part += int(num)
    print(total_not_part)
    
    part_regex = re.compile("\d+")
    part_res = part_regex.findall(input)
    total_part = - total_not_part
    for num in part_res:
        total_part += int(num)
    print(total_part)
