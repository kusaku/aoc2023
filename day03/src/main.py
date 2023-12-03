from pathlib import Path


def part1():
    schematic = Path('input.txt').read_text().splitlines()

    rows, cols = len(schematic), len(schematic[0])

    current_part_number = ''
    is_part_number = False
    part_numbers = []

    for i in range(rows):
        for j in range(cols):
            if schematic[i][j].isdecimal():
                current_part_number += schematic[i][j]

                for dx in [-1, 0, 1]:
                    for dy in [-1, 0, 1]:
                        ni, nj = i + dx, j + dy
                        if 0 <= ni < rows and 0 <= nj < cols:
                            adjancent_char = schematic[ni][nj]

                            if not (adjancent_char.isdecimal() or adjancent_char == '.'):
                                is_part_number = True

            else:
                if current_part_number and is_part_number:
                    part_numbers.append(int(current_part_number))

                current_part_number = ''
                is_part_number = False

    print(f'Answer: {sum(part_numbers)}')


def part2():
    schematic = Path('input.txt').read_text().splitlines()

    rows, cols = len(schematic), len(schematic[0])

    current_number = ''
    adjacent_gears = set()
    gear_to_ratios = {}

    for i in range(rows):
        for j in range(cols):
            if schematic[i][j].isdecimal():
                current_number += schematic[i][j]

                for dx in [-1, 0, 1]:
                    for dy in [-1, 0, 1]:
                        ni, nj = i + dx, j + dy
                        if 0 <= ni < rows and 0 <= nj < cols and schematic[ni][nj] == '*':
                            adjacent_gears.add((ni, nj))

            else:
                if current_number and adjacent_gears:
                    for gear in adjacent_gears:
                        gear_to_ratios.setdefault(gear, []).append(int(current_number))

                current_number = ''
                adjacent_gears = set()

    print(f'Answer: {sum(ratios[0] * ratios[1] for ratios in gear_to_ratios.values() if len(ratios) == 2)}')


if __name__ == '__main__':
    part1()
    part2()
