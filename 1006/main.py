from numpy import random

my_list = random.randint(0, 2, size=8 * 3)

for i in range(0, len(my_list), 8):
    row = my_list[i:i + 8]
    print(row)

true_list = []
out_list = []

for i in range(0, 3, 1):
    num = 0
    for j in range(0, 8, 1):
        num = num + my_list[j + i * 8] * (2 ** (7 - j))
    true_list.append(num)

    # -100~100
    num = num - 128
    if num > 100:
        num = 100
    elif num < -100:
        num = -100
    out_list.append(num)

print(true_list)
print(out_list)
