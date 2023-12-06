import re


input_file = 'input.txt'
output = 0

with open(input_file, 'r') as input:

    lines = input.read().split('\n')
    for line in lines:
        line = re.sub('[^\d]', '', line)
        if(len(line) < 1): continue

        number = line[0] + line[-1]
        try: 
            output += int(number)
        except ValueError as error:
            print(f"'{number}' was not a number: \n{error}")


print(output)