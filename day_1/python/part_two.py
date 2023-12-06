import re


NUMBER_MAPPINGS = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9'
}
NUMBERS_REGEX: str = f"(?=({'|'.join(NUMBER_MAPPINGS.keys())}|{'|'.join(NUMBER_MAPPINGS.values())}))"

input_file: str = 'input.txt'
output: int = 0
input_lines = []
output_lines = []


def convert_to_numbers(input_string: str) -> str:
    input = input_string.lower()
    output_list = []
    
    numbers = list(re.finditer(NUMBERS_REGEX, input))
    for number in numbers:
        number = number.group(1)
        if number.isnumeric():
            output_list.append(number)
        else:
            output_list.append(NUMBER_MAPPINGS[number])

    return ''.join(output_list)

def validate(answers_file: str):
    
    with open(answers_file, 'r') as file:
        answers = file.read().split('\n')
        for i in range(len(answers)):
           if output_lines[i] != answers[i]:
                print(f"Discrepancy found on line {i}. Original: {input_lines[i]}, Output: {output_lines[i]}, Answer: {answers[i]}")


with open(input_file, 'r') as input:

    lines: list[str] = input.read().split('\n')
    input_lines = lines
    for line_string in lines:

        line = convert_to_numbers(line_string)
        if(len(line) < 1): 
            output_lines.append('')
            continue
        
        number: str = line[0] + line[-1]
        output_lines.append(number)
        try: 
            output += int(number)
        except ValueError as error:
            print(f"'{number}' was not a number: \n{error}")

#validate('bulk_test_answers.txt')

print(output)