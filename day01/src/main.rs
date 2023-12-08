use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_TO_DIGIT1: HashMap<&'static str, usize> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    static ref WORD_TO_DIGIT2: HashMap<&'static str, usize> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

fn extract_digits(s: &str, word_to_digit: &HashMap<&str, usize>) -> Option<usize> {
    let mut digits = Vec::new();

    for i in 0..s.len() {
        for (&word, &digit) in word_to_digit.iter() {
            if s[i..].starts_with(word) {
                digits.push(digit);
                break;
            }
        }
    }

    if digits.is_empty() {
        return None;
    }

    return Some(10 * digits.first().unwrap() + digits.last().unwrap());
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let r = input
        .lines()
        .filter_map(|line| extract_digits(line, &WORD_TO_DIGIT1))
        .sum::<usize>();

    println!("Answer: {}", r);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let r = input
        .lines()
        .filter_map(|line| extract_digits(line, &WORD_TO_DIGIT2))
        .sum::<usize>();

    println!("Answer: {}", r);
}

fn main() {
    part1();
    part2();
}
