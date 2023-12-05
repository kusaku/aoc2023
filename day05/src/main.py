from pathlib import Path


def seeds_ranges1(seeds_line):
    for seed in map(int, seeds_line[6:].split()):
        yield seed, seed


def seeds_ranges2(seeds_line):
    for seeds_start, seeds_length in zip(*[map(int, seeds_line[6:].split())] * 2):
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


def part1():
    almanac = iter(Path('input.txt').read_text().splitlines())
    seeds_ranges = seeds_ranges1(next(almanac))

    for rule_line in almanac:
        if rule_line == '':
            step_name, rules = next(almanac)[:-5], []
            # chain generator
            seeds_ranges = offset_seeds(rules, seeds_ranges)
        else:
            dst_start, src_start, length = map(int, rule_line.split())
            rules.append((src_start, src_start + length, dst_start - src_start))

    print(f'Answer: {min(s[0] for s in seeds_ranges)}')


def part2():
    almanac = iter(Path('input.txt').read_text().splitlines())
    seeds_ranges = seeds_ranges2(next(almanac))

    for rule_line in almanac:
        if rule_line == '':
            step_name, rules = next(almanac)[:-5], []
            # chain generators
            seeds_ranges = offset_seeds(rules, seeds_ranges)
        else:
            dst_start, src_start, length = map(int, rule_line.split())
            rules.append((src_start, src_start + length, dst_start - src_start))

    print(f'Answer: {min(s[0] for s in seeds_ranges)}')


if __name__ == '__main__':
    part1()
    part2()
