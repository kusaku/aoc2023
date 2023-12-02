use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_TO_DIGIT1: HashMap<&'static str, i32> = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ].iter().cloned().collect();
}

lazy_static! {
    static ref WORD_TO_DIGIT2: HashMap<&'static str, i32> = [
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
    ].iter().cloned().collect();
}


fn extract_digits(s: &str, word_to_digit: &HashMap<&'static str, i32>) -> Option<i32> {
    let mut digits = Vec::new();

    for i in 0..s.len() {
        for (&word, &digit) in word_to_digit.iter() {
            if s[i..].starts_with(word) {
                digits.push(digit);
                break;
            }
        }
    }

    if !digits.is_empty() {
        return Some(10 * digits.first().unwrap() + digits.last().unwrap());
    }

    return None;
}

fn part1() {
    if let Ok(file) = File::open("input.txt") {
        let sum_result: i32 = io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| extract_digits(&line, &WORD_TO_DIGIT1))
            .sum();

        println!("Answer: {}", sum_result);
    } else {
        eprintln!("Error reading the file.");
    }
}

fn part2() {
    if let Ok(file) = File::open("input.txt") {
        let sum_result: i32 = io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| extract_digits(&line, &WORD_TO_DIGIT2))
            .sum();

        println!("Answer: {}", sum_result);
    } else {
        eprintln!("Error reading the file.");
    }
}

fn main() {
    part1();
    part2();
}
