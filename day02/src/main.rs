use std::collections::{HashMap, HashSet};
use std::fs;

use lazy_static::lazy_static;

lazy_static! {
    static ref BAG: HashMap<&'static str, u64> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
}

fn part1() {
    let mut all = HashSet::new();
    let mut bad = HashSet::new();

    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    for line in input.lines() {
        let mut iter = line.splitn(2, ':');
        let game_number = iter.next().unwrap()[5..].parse().unwrap();
        let results = iter.next().unwrap();

        all.insert(game_number);

        for result in results.split(';') {
            for cube in result.split(',') {
                let mut iter = cube.split_whitespace();
                let amount: u64 = iter.next().unwrap().parse().unwrap();
                let color: &str = iter.next().unwrap();

                if amount > BAG[color] {
                    bad.insert(game_number);
                }
            }
        }
    }

    println!("Answer: {}", all.difference(&bad).sum::<u64>());
}

fn part2() {
    let mut r = 0;

    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    for line in input.lines() {
        let mut color_amounts = vec![Vec::new(); 3]; // red, green, blue

        for result in line.splitn(2, ':').last().unwrap().split(';') {
            for cube in result.split(',') {
                let mut iter = cube.trim().split(' ');
                let amount: u64 = iter.next().unwrap().parse().unwrap();

                match iter.next().unwrap() {
                    "red" => color_amounts[0].push(amount),
                    "green" => color_amounts[1].push(amount),
                    "blue" => color_amounts[2].push(amount),
                    _ => panic!("Unexpected color"),
                }
            }
        }

        r += color_amounts
            .iter()
            .map(|cubes_amounts| cubes_amounts.iter().max().unwrap())
            .product::<u64>();
    }

    println!("Answer: {}", r);
}

fn main() {
    part1();
    part2();
}
