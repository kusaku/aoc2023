use std::fs;

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut races = input.lines();

    let times: Vec<i64> = races.next().unwrap()[5..].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let distances: Vec<i64> = races.next().unwrap()[9..].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut result = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        let d = ((time * time - 4 * distance) as f64).sqrt();
        let (x1, x2) = ((time as f64 - d) / 2.0, (time as f64 + d) / 2.0);
        result *= x2 as i64 - x1 as i64 - (x1 == x1.floor()) as i64 - (x2 == x2.floor()) as i64;
    }

    println!("Answer: {}", result);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut races = input.lines();

    let time: i64 = races.next().unwrap()[5..].split_whitespace().collect::<String>().parse().unwrap();
    let distance: i64 = races.next().unwrap()[9..].split_whitespace().collect::<String>().parse().unwrap();

    let d = ((time * time - 4 * distance) as f64).sqrt();
    let (x1, x2) = ((time as f64 - d) / 2.0, (time as f64 + d) / 2.0);
    let result = x2 as i64 - x1 as i64 - (x1 == x1.floor()) as i64 - (x2 == x2.floor()) as i64;

    println!("Answer: {}", result);
}

fn main() {
    part1();
    part2();
}
