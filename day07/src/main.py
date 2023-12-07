from collections import Counter
from dataclasses import dataclass
from pathlib import Path

@dataclass
class Row:
    bid: int
    hand_rank: int
    cards_strength: tuple[int, ...]


RANK_ORDER: dict[tuple[int, ...], int] = {
    (5,): 6,             # Five of a kind
    (4, 1): 5,           # Four of a kind
    (3, 2): 4,           # Full house
    (3, 1, 1): 3,        # Three of a kind
    (2, 2, 1): 2,        # Two pair
    (2, 1, 1, 1): 1,     # One pair
    (1, 1, 1, 1, 1): 0,  # High card
}


def part1():
    card_order = '23456789TJQKA'
    hands_and_bids = []

    for hand, bid in map(str.split, Path('input.txt').read_text().splitlines()):
        hand_type = tuple(sorted(Counter(hand).values(), reverse=True))
        cards_strength = tuple(card_order.find(card) for card in hand)
        hands_and_bids.append(Row(int(bid), RANK_ORDER[hand_type], cards_strength))

    r = sum(
        rank * row.bid
        for rank, row
        in enumerate(sorted(hands_and_bids, key=lambda row: (row.hand_rank, row.cards_strength)), 1)
    )

    print(f'Answer: {r}')


def part2():
    card_order = 'J23456789TQKA'
    hands_and_bids = []

    for hand, bid in map(str.split, Path('input.txt').read_text().splitlines()):
        cards_strength = tuple(card_order.find(card) for card in hand)

        if 'J' in hand and hand != 'JJJJJ':
            c = Counter(s for s in cards_strength if s != 0)
            if len(c) >= 2:
                Ellipsis
            best_j_replacement = card_order[max(i for i, n in c.most_common() if n == c.most_common(1)[0][1])]
            virtual_hand = hand.replace('J', best_j_replacement)
        else:
            virtual_hand = hand

        hand_type = tuple(sorted(Counter(virtual_hand).values(), reverse=True))
        cards_strength = tuple(card_order.find(card) for card in hand)

        hands_and_bids.append(Row(int(bid), RANK_ORDER[hand_type], cards_strength))

    r = sum(
        rank * row.bid
        for rank, row
        in enumerate(sorted(hands_and_bids, key=lambda row: (row.hand_rank, row.cards_strength)), 1)
    )

    print(f'Answer: {r}')


if __name__ == '__main__':
    part1()
    part2()
