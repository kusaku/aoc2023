use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_TO_DIGIT: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        map.insert("3", 3);
        map.insert("4", 4);
        map.insert("5", 5);
        map.insert("6", 6);
        map.insert("7", 7);
        map.insert("8", 8);
        map.insert("9", 9);
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);
        map
    };
}

fn extract_digits(s: &str) -> Option<i32> {
    let mut digits = Vec::new();

    for i in 0..s.len() {
        for (&word, &digit) in WORD_TO_DIGIT.iter() {
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

fn main() {
    if let Ok(file) = File::open("01_input.txt") {
        let sum_result: i32 = io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| extract_digits(&line))
            .sum();

        println!("Sum: {}", sum_result);
    } else {
        eprintln!("Error reading the file.");
    }
}
