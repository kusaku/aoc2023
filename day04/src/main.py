from pathlib import Path


def part1():
    cards = Path('input.txt').read_text().splitlines()
    total_points = 0

    for card in cards:
        game, numbers = card.split(':', maxsplit=1)
        win_numbers, my_numbers = numbers.split('|')
        win_numbers, my_numbers = set(win_numbers.split()), set(my_numbers.split())
        number_of_matches = len(win_numbers & my_numbers)
        points = 2 ** (number_of_matches - 1) if number_of_matches else 0
        total_points += points

    print(f'Answer: {total_points}')


def part2():
    cards = Path('input.txt').read_text().splitlines()
    cards_counter = {i + 1: 1 for i in range(len(cards))}

    for card in cards:
        card_info, numbers = card.split(':', maxsplit=1)
        card_number = int(card_info[5:])
        win_numbers, my_numbers = numbers.split('|')
        win_numbers, my_numbers = set(win_numbers.split()), set(my_numbers.split())
        number_of_matches = len(win_numbers & my_numbers)
        numer_of_cards = cards_counter[card_number]
        for i in range(number_of_matches):
            cards_counter[card_number + i + 1] += numer_of_cards

    print(f'Answer: {sum(cards_counter.values())}')


if __name__ == '__main__':
    part1()
    part2()
