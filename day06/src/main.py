from pathlib import Path


def part1():
    races = iter(Path('input.txt').read_text().splitlines())
    times, distances = map(int, next(races)[5:].split()), map(int, next(races)[9:].split())

    r = 1
    for time, distance in zip(times, distances):
        d = (time * time - 4 * distance) ** 0.5
        x1, x2 = (time - d) / 2, (time + d) / 2
        r *= int(x2) - int(x1) - (x1 == int(x1)) - (x2 == int(x2))

    print(f'Answer: {r}')


def part2():
    races = iter(Path('input.txt').read_text().splitlines())
    time, distance = int(next(races)[5:].replace(' ', '')), int(next(races)[9:].replace(' ', ''))
    d = (time * time - 4 * distance) ** 0.5
    x1, x2 = (time - d) / 2, (time + d) / 2
    r = int(x2) - int(x1) - (x1 == int(x1)) - (x2 == int(x2))

    print(f'Answer: {r}')


if __name__ == '__main__':
    part1()
    part2()
