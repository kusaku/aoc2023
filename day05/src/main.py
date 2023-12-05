from pathlib import Path


def seeds_ranges1(seeds_line):
    for seed in map(int, seeds_line[6:].split()):
        yield seed, seed


def seeds_ranges2(seeds_line):
    for seeds_start, seeds_length in zip(*[map(int, seeds_line[6:].split())] * 2):
        # for seed in range(seeds_start, seeds_start + seeds_length):
        #     yield seed, seed
        yield seeds_start, seeds_start + seeds_length - 1


def offset_seeds(rules, seeds_ranges):
    for seeds_start, seeds_end in seeds_ranges:
        # apply offsets
        for range_start, range_end, offset in rules:
            # seeds inside range
            if seeds_start >= range_start and seeds_end <= range_end:
                yield seeds_start + offset, seeds_end + offset
                break
            # range inside seeds
            elif seeds_start < range_start and seeds_end > range_end:
                yield seeds_start, range_start - 1
                yield range_start + offset, range_end - 1 + offset
                yield range_end, seeds_end
                break
            # seeds intersect with range on the left
            elif seeds_start <= range_start <= seeds_end <= range_end:
                yield seeds_start, range_start - 1
                yield range_start + offset, seeds_end + offset
                break
            # seeds intersect with range on the right
            elif range_start <= seeds_start <= range_end <= seeds_end:
                yield seeds_start, range_end - 1
                yield range_end + offset, seeds_end + offset
                break
            # seeds and range do not intersect
            else:
                assert range_end < seeds_start or seeds_end < range_start
        else:
            # no offsets were applied
            yield seeds_start, seeds_end


def part12():
    almanac = iter(Path('input.txt').read_text().splitlines())
    seeds_line = next(almanac)
    seeds_ranges = seeds_ranges2(seeds_line)
    # first_seed_range = next(seeds_ranges2(seeds_line))
    # print(first_seed_range)
    # print()
    # seeds_ranges = iter([first_seed_range])

    # seeds_ranges = iter([(82, 82)])

    # print(list(gen_seeds2(seed_line)))

    steps = {}

    for rule_line in almanac:
        if rule_line == '':
            step_name = next(almanac)[:-5]
        else:
            dst_start, src_start, length = map(int, rule_line.split())
            rules = steps.setdefault(step_name, [])
            rules.append((src_start, src_start + length, dst_start - src_start))

    for step_name, rules in steps.items():
        # seeds_ranges = list(seeds_ranges)
        # print(f'{seeds_ranges} -> ', end='')
        # seeds_ranges = iter(seeds_ranges)
        seeds_ranges = offset_seeds(rules, seeds_ranges)
        # seeds_ranges = list(seeds_ranges)
        # print(f'{step_name} - {seeds_ranges}')
        # seeds_ranges = iter(seeds_ranges)

    # seeds_ranges = list(seeds_ranges)
    # for s in seeds_ranges:
    #     print(s)

    # part1, sample - 35, input - 199602917
    # part1, sample - 46, input - 2254686 (direct search)
    print(f'Answer: {min(s[0] for s in seeds_ranges)}')


if __name__ == '__main__':
    part12()
