use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use lazy_static::lazy_static;

lazy_static! {
    static ref BAG: HashMap<&'static str, u64> = [("red", 12), ("green", 13), ("blue", 14),]
        .into_iter()
        .collect();
}

fn part1() {
    let mut all = HashSet::new();
    let mut bad = HashSet::new();

    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(entry) = line {
                let mut iter = entry.split(':');
                let game_number = iter.next().unwrap()[5..].parse().unwrap();
                let results = iter.next().unwrap();

                all.insert(game_number);

                for result in results.split(';') {
                    let cubes: Vec<&str> = result.split(',').collect();

                    for cube in cubes {
                        let mut iter = cube.trim().split(' ');
                        let amount: u64 = iter.next().unwrap().parse().unwrap();
                        let color: &str = iter.next().unwrap();

                        if amount > *BAG.get(color).unwrap() {
                            bad.insert(game_number);
                        }
                    }
                }
            }
        }
        println!("Answer: {}", all.difference(&bad).sum::<u64>());
    } else {
        eprintln!("Error reading the file.");
    }
}

fn part2() {
    let mut r = Vec::new();

    if let Ok(file) = File::open("input.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(entry) = line {
                if let Some(results) = entry.split(':').nth(1) {
                    let mut color_amounts = vec![Vec::new(); 3]; // red, green, blue

                    for result in results.split(';') {
                        let cubes: Vec<&str> = result.split(',').collect();

                        for cube in cubes {
                            let mut iter = cube.trim().split(' ');
                            let amount: u64 = iter.next().unwrap().parse().unwrap();
                            let color: &str = iter.next().unwrap();
                            match color {
                                "red" => color_amounts[0].push(amount),
                                "green" => color_amounts[1].push(amount),
                                "blue" => color_amounts[2].push(amount),
                                _ => panic!("Unexpected color"),
                            }
                        }
                    }

                    r.push(
                        color_amounts
                            .iter()
                            .map(|cubes_amounts| cubes_amounts.iter().max().unwrap())
                            .product(),
                    );
                }
            }
        }

        println!("Answer: {}", r.iter().sum::<u64>());
    } else {
        eprintln!("Failed to open the file.");
    }
}

fn main() {
    part1();
    part2();
}
