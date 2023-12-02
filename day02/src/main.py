from functools import reduce

BAG = {
    'red': 12,
    'green': 13,
    'blue': 14,
}

def part1() -> None:
    all = set()
    bad = set()

    with open('input.txt') as file:
        for line in file:
            game, results = line.split(':', maxsplit=1)
            game_number = int(game[5:])
            all.add(game_number)
            for result in results.split(';'):
                cubes = result.split(',')
                for cube in cubes:
                    amount, color = cube.strip().split(' ')
                    amount = int(amount)
                    if amount > BAG[color]:
                        bad.add(game_number)

    print(f'Answer: {sum(all - bad)}')


def part2() -> None:
    all = []

    with open('input.txt') as file:
        for line in file:
            _, results = line.split(':', maxsplit=1)
            amounts = {'red': [], 'green': [], 'blue': [],}
            for result in results.split(';'):
                cubes = result.split(',')
                for cube in cubes:
                    amount, color = cube.strip().split(' ')
                    amount = int(amount)
                    amounts[color].append(amount)

            all.append(reduce((lambda x, y: x * y), map(max, amounts.values())))

    print(f'Answer: {sum(all)}')

if __name__ == '__main__':
    part1()
    part2()
