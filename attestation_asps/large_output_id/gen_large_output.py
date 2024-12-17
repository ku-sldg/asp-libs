# a function that will take the input number and pad it with "#" at the beginning to make it 4 characters long
def pad_number(number):
    return "#" * (4 - len(str(number))) + str(number)


INPUT_LEN = 1024

for i in range(INPUT_LEN):
    print(pad_number(i), end="")
