WORDS_TO_REPLACE = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9',
}

def main() -> None:
    with open('01_debug.txt', 'r') as file:
        sum_result = sum(extract_digits(s[:-1]) for s in file)

    print('Sum:', sum_result)


def extract_digits(s: str) -> int:
    r = ''

    for c in s:
        r += c

        for word, numeric_value in WORDS_TO_REPLACE.items():
            if r.endswith(word):
                r = r[:-len(word)] + numeric_value

    digits = [c for c in r if c.isdigit()]

    if digits:
        print(f'{s} -> {r} -> {digits[0]}{digits[-1]}')
        return int(f'{digits[0]}{digits[-1]}')

    print(f'{s} -> {r} -> None')
    return 0


if __name__ == '__main__':
    main()
