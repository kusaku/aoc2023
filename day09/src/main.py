from pathlib import Path


def part1():
    res = 0

    for line in Path('input.txt').read_text().splitlines():
        series = list(map(int, line.split()))
        res += series[-1]

        while any(series):
            series = [b - a for a, b in zip(series, series[1:])]
            res += series[-1]

    print(f'Answer: {res}')


def part2():
    res = 0

    for line in Path('input.txt').read_text().splitlines():
        series = list(map(int, line.split()))[::-1]
        res += series[-1]

        while any(series):
            series = [b - a for a, b in zip(series, series[1:])]
            res += series[-1]

    print(f'Answer: {res}')


if __name__ == '__main__':
    part1()
    part2()
