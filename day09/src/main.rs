use std::fs;

fn part1() {
    let mut res = 0;

    for line in fs::read_to_string("input.txt").expect("Error reading file").lines() {
        let mut series: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        res += series[series.len() - 1];

        while series.iter().any(|&x| x != 0) {
            series = series.windows(2).map(|x| x[1] - x[0]).collect();
            res += series[series.len() - 1];
        }
    }

    println!("Answer: {}", res);
}

fn part2() {
    let mut res = 0;

    for line in fs::read_to_string("input.txt").expect("Error reading file").lines() {
        let mut series: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).rev().collect();
        res += series[series.len() - 1];

        while series.iter().any(|&x| x != 0) {
            series = series.windows(2).map(|x| x[1] - x[0]).collect();
            res += series[series.len() - 1];
        }
    }

    println!("Answer: {}", res);
}

fn main() {
    part1();
    part2();
}
