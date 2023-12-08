import math
from itertools import cycle
from pathlib import Path


def part1():
    data = Path('input.txt').read_text().splitlines()
    dirs = data[0]
    the_map = {}

    for line in data[2:]:
        node, fork = line.split(' = ')
        the_map[node] = {k: v for k, v in zip(['L', 'R'], fork[1:-1].split(', '))}

    curr = 'AAA'

    for steps, dir in enumerate(cycle(dirs)):
        if curr == 'ZZZ':
            break
        curr = the_map[curr][dir]

    print(f'Answer: {steps}')


def part2():
    data = Path('input.txt').read_text().splitlines()
    dirs = data[0]
    the_map = {}

    for line in data[2:]:
        node, fork = line.split(' = ')
        the_map[node] = {k: v for k, v in zip(['L', 'R'], fork[1:-1].split(', '))}

    least_steps = []

    for curr in [node for node in the_map.keys() if node[2] == 'A']:
        for steps, dir in enumerate(cycle(dirs)):
            if curr[2] == 'Z':
                least_steps.append(steps)
                break
            curr = the_map[curr][dir]

    print(f'Answer: {math.lcm(*least_steps)}')


if __name__ == '__main__':
    part1()
    part2()
