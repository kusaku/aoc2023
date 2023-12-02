use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS_TO_REPLACE: HashMap<&'static str, char> = {
        let mut map = HashMap::new();
        map.insert("one", '1');
        map.insert("two", '2');
        map.insert("three", '3');
        map.insert("four", '4');
        map.insert("five", '5');
        map.insert("six", '6');
        map.insert("seven", '7');
        map.insert("eight", '8');
        map.insert("nine", '9');
        map
    };
}

fn main() -> io::Result<()> {
    let file_path = "01_debug.txt";
    let file = File::open(file_path)?;

    let sum: u32 = io::BufReader::new(file).lines()
                                           .filter_map(|line| line.ok().and_then(|s| extract_digits(&s)))
                                           .map(|(first, last)| format!("{}{}", first, last).parse::<u32>().ok())
                                           .filter_map(|parsed_number| parsed_number)
                                           .sum();

    println!("Sum: {}", sum);

    Ok(())
}

fn extract_digits(s: &str) -> Option<(char, char)> {
    print!("{}", s);

    let mut r = String::new();

    for c in s.chars() {
        r.push(c);
        for (word, &numeric_value) in WORDS_TO_REPLACE.iter() {
            if r.ends_with(word) {
                r.drain(r.len() - word.len()..);
                r.push(numeric_value);
            }
        }
    }

    print!(" -> {}", r);

    let digits: Vec<_> = r.chars().filter(|c| c.is_digit(10)).collect();

    if !digits.is_empty() {
        let first = *digits.first().unwrap();
        let last = *digits.last().unwrap();
        println!(" -> {}{}", first, last);
        return Some((first, last));
    }

    println!(" -> None");

    None
}
