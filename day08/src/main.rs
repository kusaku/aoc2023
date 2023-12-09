use std::collections::HashMap;
use std::fs;

use num_integer::Integer;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Error reading file");
    let data: Vec<&str> = content.lines().collect();
    let dirs: &str = data[0];
    let mut the_map: HashMap<&str, HashMap<&char, &str>> = HashMap::new();

    for line in data.iter().skip(2) {
        let mut parts = line.splitn(2, " = ");
        let node = parts.next().unwrap();
        let next = parts.next().unwrap();
        let mut nexts = next[1..next.len() - 1].splitn(2, ", ");
        let l = nexts.next().unwrap();
        let r = nexts.next().unwrap();
        the_map.insert(node, HashMap::from([(&'L', l), (&'R', r), ]));
    }

    // println!("Answer: {:?}", the_map);

    let mut curr = "AAA";
    let mut steps = 0;

    for (i, dir) in dirs.chars().cycle().enumerate() {
        if curr == "ZZZ" {
            break;
        }
        curr = the_map[&curr][&dir];
        steps = i + 1;
    }

    println!("Answer: {}", steps);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Error reading file");
    let data: Vec<&str> = content.lines().collect();
    let dirs: &str = data[0];
    let mut the_map: HashMap<&str, HashMap<&char, &str>> = HashMap::new();

    for line in data.iter().skip(2) {
        let mut parts = line.splitn(2, " = ");
        let node = parts.next().unwrap();
        let next = parts.next().unwrap();
        let mut nexts = next[1..next.len() - 1].splitn(2, ", ");
        let l = nexts.next().unwrap();
        let r = nexts.next().unwrap();
        the_map.insert(node, HashMap::from([(&'L', l), (&'R', r), ]));
    }

    let mut least_steps: Vec<usize> = Vec::new();
    let mut curr: &str;

    for &node in the_map.keys().filter(|&&node| node.ends_with('A')) {
        let mut steps = 0;
        curr = node;

        for (i, dir) in dirs.chars().cycle().enumerate() {
            if curr.ends_with('Z') {
                least_steps.push(steps);
                break;
            }
            curr = the_map[&curr][&dir];
            steps = i + 1;
        }
    }

    println!("Answer: {}", least_steps.iter().fold(least_steps[0], |acc, x| acc.lcm(x)));
}

fn main() {
    part1();
    part2();
}
