WORD_TO_DIGIT = {
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


def extract_digits(s: str) -> int:
    digits = [digit for i in range(0, len(s)) for word, digit in WORD_TO_DIGIT.items() if s[i:].startswith(word)]

    result = 10 * digits[0] + digits[-1] if digits else None

    return result


def main() -> None:
    with open('01_input.txt') as file:
        sum_result = sum(extract_digits(line) for line in file)

    print('Sum:', sum_result)


if __name__ == '__main__':
    main()
