from itertools import combinations
from pathlib import Path


def parse_universe(data):
    lines = data.strip().splitlines()
    galaxy_positions = {
        (x, y) for y, line in enumerate(lines)
        for x, char in enumerate(line) if char == '#'
    }
    universe_height = len(lines)
    universe_width = len(lines[0])

    return (universe_width, universe_height), galaxy_positions


def display_universe(dimensions, galaxy_positions):
    universe_width, universe_height = dimensions
    for y in range(universe_height):
        for x in range(universe_width):
            print('#' if (x, y) in galaxy_positions else '.', end='')

        print()


def scale_universe(dimensions, galaxy_positions, scale_factor):
    universe_width, universe_height = dimensions

    def scale_coordinates(coordinates, max_size):
        mapping = {}
        scaled_total = 0
        previous_coordinate = 0
        for coordinate in coordinates:
            scaled_total += (coordinate - previous_coordinate) * scale_factor
            mapping[coordinate] = scaled_total
            scaled_total += 1
            previous_coordinate = coordinate + 1
        scaled_total += (max_size - previous_coordinate) * scale_factor

        return mapping, scaled_total

    x_coordinates = sorted({x for x, _ in galaxy_positions})
    y_coordinates = sorted({y for _, y in galaxy_positions})

    x_mapping, scaled_width = scale_coordinates(x_coordinates, universe_width)
    y_mapping, scaled_height = scale_coordinates(y_coordinates, universe_height)

    scaled_positions = {
        (x_mapping[x], y_mapping[y]) for x, y in galaxy_positions
    }

    return (scaled_width, scaled_height), scaled_positions


def calculate_total_manhattan_distance(galaxy_positions):
    return sum(
        abs(x1 - x2) + abs(y1 - y2)
            for (x1, y1), (x2, y2) in combinations(galaxy_positions, 2)
    )


def part1():
    data = Path('input.txt').read_text().strip()
    dimensions, galaxy_positions = parse_universe(data)

    # print('Original Universe:')
    # display_universe(dimensions, galaxy_positions)

    dimensions, galaxy_positions = scale_universe(dimensions, galaxy_positions, scale_factor=2)

    # print('Scaled Universe:')
    # display_universe(dimensions, galaxy_positions)

    total_distance = calculate_total_manhattan_distance(galaxy_positions)
    print(f'Answer: {total_distance}')


def part2():
    data = Path('input.txt').read_text().strip()
    dimensions, galaxy_positions = parse_universe(data)

    # print('Original Universe:')
    # display_universe(dimensions, galaxy_positions)

    dimensions, galaxy_positions = scale_universe(dimensions, galaxy_positions, scale_factor=1_000_000)

    # print('Scaled Universe:')
    # display_universe(dimensions, galaxy_positions)

    total_distance = calculate_total_manhattan_distance(galaxy_positions)
    print(f'Answer: {total_distance}')


if __name__ == '__main__':
    part1()
    part2()
