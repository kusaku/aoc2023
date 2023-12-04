use std::collections::HashMap;
use std::fs;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");
    let cards: Vec<&str> = content.lines().collect();
    let mut total_points = 0;

    for card in cards {
        let mut parts = card.splitn(2, ':');
        let numbers = parts.nth(1).unwrap();
        let mut numbers_parts = numbers.split('|');
        let win_numbers: Vec<&str> = numbers_parts.next().unwrap().split_whitespace().collect();
        let my_numbers: Vec<&str> = numbers_parts.next().unwrap().split_whitespace().collect();
        let number_of_matches = win_numbers.iter().filter(|&x| my_numbers.contains(x)).count();
        let points = if number_of_matches > 0 { 1 << (number_of_matches - 1) } else { 0 };

        total_points += points;
    }

    println!("Answer: {}", total_points);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");
    let cards: Vec<&str> = content.lines().collect();
    let mut cards_counter: HashMap<usize, usize> = HashMap::new();

    for i in 0..cards.len() {
        cards_counter.insert(i + 1, 1);
    }

    for card in cards {
        let mut card_parts = card.splitn(2, ':');
        let card_info = card_parts.next().unwrap();
        let numbers = card_parts.next().unwrap();
        let card_number = card_info
            .trim_start_matches("Card")
            .trim()
            .parse::<usize>()
            .expect("Failed to parse card number");
        let mut numbers_parts = numbers.split('|');
        let win_numbers: Vec<&str> = numbers_parts.next().unwrap().split_whitespace().collect();
        let my_numbers: Vec<&str> = numbers_parts.next().unwrap().split_whitespace().collect();
        let number_of_matches = win_numbers.iter().filter(|x| my_numbers.contains(x)).count();
        let numer_of_cards = cards_counter[&card_number];

        for i in 0..number_of_matches {
            *cards_counter.entry(card_number + i + 1).or_insert(0) += numer_of_cards;
        }
    }

    println!("Answer: {}", cards_counter.values().sum::<usize>());
}


fn main() {
    part1();
    part2();
}
