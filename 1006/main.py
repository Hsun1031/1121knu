from numpy import random

my_list = random.randint(0, 2, size=8 * 3)

true_list = []
out_list = []

def to_hex(list):
    num = 0
    for i in range(0, 8, 1):
        num = num + list[i] * (2 ** (7 - i))
    return num

for i in range(0, len(my_list), 8):
    row = my_list[i:i + 8]
    print(row)
    num = to_hex(row)
    true_list.append(num)



for num in true_list:
    # -100~100
    num = num - 128
    if num > 100:
        num = 100
    elif num < -100:
        num = -100
    out_list.append(num)

print(true_list)
print(out_list)
