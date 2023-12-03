use std::collections::{HashMap, HashSet};
use std::fs;

const OFFSETS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let schematic: Vec<&str> = input.lines().collect();

    let rows = schematic.len();
    let cols = schematic[0].len();

    let mut current_part_number = String::new();
    let mut is_part_number = false;
    let mut part_numbers = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            let current_char = schematic[i].chars().nth(j).unwrap();

            if current_char.is_digit(10) {
                current_part_number.push(current_char);

                for (dx, dy) in OFFSETS {
                    let (ni, nj) = (i.wrapping_add_signed(dx), j.wrapping_add_signed(dy));

                    if ni < rows && nj < cols {
                        let adjacent_char = schematic[ni].chars().nth(nj).unwrap();

                        if !adjacent_char.is_digit(10) && adjacent_char != '.' {
                            is_part_number = true;
                        }
                    }
                }
            } else {
                if !current_part_number.is_empty() && is_part_number {
                    part_numbers.push(current_part_number.parse::<i32>().unwrap());
                }

                current_part_number.clear();
                is_part_number = false;
            }
        }
    }

    println!("Answer: {}", part_numbers.iter().sum::<i32>());
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let schematic: Vec<&str> = input.lines().collect();

    let rows = schematic.len();
    let cols = schematic[0].len();

    let mut current_number = String::new();
    let mut adjacent_gears = HashSet::new();
    let mut gear_to_ratios: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            let current_char = schematic[i].chars().nth(j).unwrap();

            if current_char.is_digit(10) {
                current_number.push(current_char);

                for (dx, dy) in OFFSETS {
                    let (ni, nj) = (i.wrapping_add_signed(dx), j.wrapping_add_signed(dy));

                    if ni < rows && nj < cols && schematic[ni].chars().nth(nj).unwrap() == '*' {
                        adjacent_gears.insert((ni, nj));
                    }
                }
            } else {
                if !current_number.is_empty() && !adjacent_gears.is_empty() {
                    for &gear in &adjacent_gears {
                        gear_to_ratios
                            .entry(gear)
                            .or_insert(Vec::new())
                            .push(current_number.parse().unwrap());
                    }
                }

                current_number.clear();
                adjacent_gears.clear();
            }
        }
    }

    let answer: i32 = gear_to_ratios
        .values()
        .filter(|ratios| ratios.len() == 2)
        .map(|ratios| ratios[0] * ratios[1])
        .sum();

    println!("Answer: {}", answer);
}

fn main() {
    part1();
    part2();
}
