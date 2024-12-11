from itertools import batched
from pathlib import Path

def split_and_shift(s_start, s_length, dest_start, src_start, src_length):
    src_end = src_start + src_length - 1
    s_end = s_start + s_length - 1
    shifted = []
    remaining = []

    if s_end < src_start or s_start > src_end:
        remaining.append((s_start, s_length))
    else:
        if s_start < src_start:
            remaining.append((s_start, src_start - s_start))

        if s_end > src_end:
            remaining.append((src_end + 1, s_end - src_end))

        overlap_start = max(s_start, src_start)
        overlap_end = min(s_end, src_end)
        shifted.append((dest_start + overlap_start - src_start, overlap_end - overlap_start + 1))

    return remaining, shifted

def offset_seeds(rules, seed_ranges):
    new_ranges_seeds = []

    for seed_range in seed_ranges:
        remaining_ranges = [seed_range]
        shifted_ranges = []

        for dest_start, src_start, src_length in rules:
            updated_ranges = []

            for s_start, s_length in remaining_ranges:
                rem, shift = split_and_shift(s_start, s_length, dest_start, src_start, src_length)
                updated_ranges.extend(rem)
                shifted_ranges.extend(shift)

            remaining_ranges = updated_ranges

        new_ranges_seeds.extend(remaining_ranges)
        new_ranges_seeds.extend(shifted_ranges)

    return new_ranges_seeds

def part1():
    seed_sec, *rule_secs = Path('input.txt').read_text().strip().split("\n\n")

    seed_ranges = [(s_start, 1) for s_start in map(int, seed_sec.split("seeds:")[1].split())]

    for rule_sec in rule_secs:
        rules = [tuple(map(int, line.split())) for line in rule_sec.strip().split("\n")[1:]]
        seed_ranges = offset_seeds(rules, seed_ranges)

    print(f'Answer: {min(s_start for s_start, s_length in seed_ranges)}')

def part2():
    seed_sec, *rule_secs = Path('input.txt').read_text().strip().split("\n\n")

    seed_ranges = [
        (s_start, s_length)
        for s_start, s_length in batched(map(int, seed_sec.split("seeds:")[1].split()), 2)
    ]

    for rule_sec in rule_secs:
        rules = [tuple(map(int, line.split())) for line in rule_sec.strip().split("\n")[1:]]
        seed_ranges = offset_seeds(rules, seed_ranges)

    print(f'Answer: {min(s_start for s_start, s_length in seed_ranges)}')

if __name__ == '__main__':
    part1()
    part2()
