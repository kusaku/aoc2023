WORD_TO_DIGIT1 = {
    '1': 1,
    '2': 2,
    '3': 3,
    '4': 4,
    '5': 5,
    '6': 6,
    '7': 7,
    '8': 8,
    '9': 9,
}

WORD_TO_DIGIT2 = {
    '1': 1,
    '2': 2,
    '3': 3,
    '4': 4,
    '5': 5,
    '6': 6,
    '7': 7,
    '8': 8,
    '9': 9,
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
}


def extract_digits(s: str, word_to_digit1: dict[str, int]) -> int:
    digits = [digit for i in range(0, len(s)) for word, digit in word_to_digit1.items() if s[i:].startswith(word)]

    result = 10 * digits[0] + digits[-1] if digits else None

    return result


def part1() -> None:
    with open('input.txt') as file:
        sum_result = sum(extract_digits(line, WORD_TO_DIGIT1) for line in file)

    print('Answer:', sum_result)


def part2() -> None:
    with open('input.txt') as file:
        sum_result = sum(extract_digits(line, WORD_TO_DIGIT2) for line in file)

    print('Answer:', sum_result)


if __name__ == '__main__':
    part1()
    part2()
