import random
import re

NUMBER_MAPPINGS = {
    '1': 'one',
    '2': 'two',
    '3': 'three',
    '4': 'four',
    '5': 'five',
    '6': 'six',
    '7': 'seven',
    '8': 'eight',
    '9': 'nine'
}
NUMBERS = list(NUMBER_MAPPINGS.values())
NUMBERS.extend(['1', '2', '3', '4', '5', '6', '7', '8', '9'])

SPELLED_NUMBERS_REGEX: str = '(' + ('|'.join(NUMBER_MAPPINGS.values())) + ')'

NUMBER_OF_LINES = 100000
MAX_JUNK_LENGTH = 15
JUNK_CHARS = "abcdefghijklmnopqrstuvwxyz"

def generate_junk(include_numbers: bool) -> str:
    output = ''
    if random.random() > 0.8: return output
    for i in range(1, random.randint(1, MAX_JUNK_LENGTH)):
        junk_character = random.choice(JUNK_CHARS)
        junk_number = random.choice(NUMBERS)
        if include_numbers:
            output += random.choice([junk_character, junk_character, junk_character, junk_number])
        else:
            output += junk_character
    if not include_numbers:
        return re.sub(SPELLED_NUMBERS_REGEX, 'l', output)
    return output

with open('bulk_test.txt', 'w') as file:
    with open('bulk_test_answers.txt', 'w') as answers:
        for i in range(NUMBER_OF_LINES):
            start_number = str(random.randint(1, 9))
            end_number = str(random.randint(1, 9))
            answers.write(start_number + end_number + '\n')

            if random.random() > 0.8: start_number = NUMBER_MAPPINGS[start_number]
            if random.random() > 0.8: end_number = NUMBER_MAPPINGS[end_number]
            start_junk = generate_junk(False)
            middle_junk = generate_junk(True)
            end_junk = generate_junk(False)

            file.write(start_junk + start_number + middle_junk + end_number + end_junk + '\n')