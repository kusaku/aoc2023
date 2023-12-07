use std::collections::HashMap;
use std::fs;

use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;

struct Row {
    bid: u64,
    hand_rank: usize,
    cards_strength: Vec<usize>,
}

lazy_static! {
    static ref RANK_ORDER: HashMap<Vec<usize>, usize> = [
        (vec![5], 6),             // Five of a kind
        (vec![4, 1], 5),          // Four of a kind
        (vec![3, 2], 4),          // Full house
        (vec![3, 1, 1], 3),       // Three of a kind
        (vec![2, 2, 1], 2),       // Two pair
        (vec![2, 1, 1, 1], 1),    // One pair
        (vec![1, 1, 1, 1, 1], 0), // High card
    ]
    .into_iter()
    .collect();
}

fn part1() {
    let card_order = "23456789TJQKA";
    let mut hands_and_bids: Vec<Row> = Vec::new();

    for line in fs::read_to_string("input.txt").expect("Error reading the file").lines() {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let counter = hand.chars().collect::<Counter<char>>();
        let hand_type = counter.values().into_iter().sorted().rev().cloned().collect::<Vec<usize>>();
        hands_and_bids.push(
            Row {
                bid: parts.next().expect("Invalid input: no bid").parse().unwrap(),
                hand_rank: RANK_ORDER[&hand_type],
                cards_strength: hand.chars().map(|c| card_order.find(c).unwrap()).collect::<Vec<usize>>(),
            }
        );
    }

    hands_and_bids.sort_by(|a, b| {
        a.hand_rank.cmp(&b.hand_rank).then(a.cards_strength.cmp(&b.cards_strength))
    });

    let r: u64 = hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, row)| (rank + 1) as u64 * row.bid)
        .sum();

    println!("Answer: {}", r);
}

fn part2() {
    let card_order = "J23456789TQKA";
    let mut hands_and_bids: Vec<Row> = Vec::new();

    for line in fs::read_to_string("input.txt").expect("Error reading the file").lines() {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap().to_string();
        let virtual_hand;

        if hand.contains('J') && hand != "JJJJJ" {
            let counter = hand.replace('J', "").chars().map(|c| card_order.find(c).unwrap()).collect::<Counter<usize>>();
            let sss = counter.into_iter().sorted_by(
                |&a, &b|
                    b.1.cmp(&a.1).then(b.0.cmp(&a.0))
            ).collect::<Vec<(usize, usize)>>();
            let best_j_replacement = card_order.chars().nth(sss.first().unwrap().0).unwrap();
            virtual_hand = hand.replace('J', &best_j_replacement.to_string());
        } else {
            virtual_hand = hand.clone();
        }

        let counter = virtual_hand.chars().collect::<Counter<char>>();
        let hand_type = counter.values().into_iter().sorted().rev().cloned().collect::<Vec<usize>>();

        hands_and_bids.push(
            Row {
                bid: parts.next().expect("Invalid input: no bid").parse().unwrap(),
                hand_rank: RANK_ORDER[&hand_type],
                cards_strength: hand.chars().map(|c| card_order.find(c).unwrap()).collect::<Vec<usize>>(),
            }
        );
    }

    hands_and_bids.sort_by(|a, b| {
        a.hand_rank.cmp(&b.hand_rank).then(a.cards_strength.cmp(&b.cards_strength))
    });

    let r: u64 = hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, row)| (rank + 1) as u64 * row.bid)
        .sum();

    println!("Answer: {}", r);
}

fn main() {
    part1();
    part2();
}
