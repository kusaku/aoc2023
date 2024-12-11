use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type Dimensions = (usize, usize);
type GalaxyPositions = HashSet<(usize, usize)>;

fn parse_universe(data: &str) -> (Dimensions, GalaxyPositions) {
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let galaxy_positions: GalaxyPositions = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| (char == '#').then_some((x, y)))
        })
        .collect();

    ((width, height), galaxy_positions)
}

// fn display_universe(dimensions: Dimensions, galaxy_positions: &GalaxyPositions) {
//     let (width, height) = dimensions;
//     for y in 0..height {
//         for x in 0..width {
//             if galaxy_positions.contains(&(x, y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn scale_universe(
    dimensions: Dimensions,
    galaxy_positions: &GalaxyPositions,
    scale_factor: usize,
) -> (Dimensions, GalaxyPositions) {
    let (width, height) = dimensions;

    fn scale_coordinates(
        coordinates: Vec<usize>,
        max_size: usize,
        scale_factor: usize,
    ) -> (Vec<usize>, usize) {
        let mut mapping = vec![0; max_size];
        let mut scaled_total = 0;
        let mut previous_coordinate = 0;

        for &coordinate in &coordinates {
            scaled_total += (coordinate - previous_coordinate) * scale_factor;
            mapping[coordinate] = scaled_total;
            scaled_total += 1;
            previous_coordinate = coordinate + 1;
        }
        scaled_total += (max_size - previous_coordinate) * scale_factor;

        (mapping, scaled_total)
    }

    let x_coordinates: Vec<_> = galaxy_positions
        .iter()
        .map(|&(x, _)| x)
        .unique()
        .sorted()
        .collect();
    let y_coordinates: Vec<_> = galaxy_positions
        .iter()
        .map(|&(_, y)| y)
        .unique()
        .sorted()
        .collect();

    let (x_mapping, scaled_width) = scale_coordinates(x_coordinates, width, scale_factor);
    let (y_mapping, scaled_height) = scale_coordinates(y_coordinates, height, scale_factor);

    let scaled_positions: GalaxyPositions = galaxy_positions
        .iter()
        .map(|&(x, y)| (x_mapping[x], y_mapping[y]))
        .collect();

    ((scaled_width, scaled_height), scaled_positions)
}

fn calculate_total_manhattan_distance(galaxy_positions: &GalaxyPositions) -> usize {
    let positions: Vec<&(usize, usize)> = galaxy_positions.iter().collect();
    positions
        .iter()
        .enumerate()
        .map(|(i, &(x1, y1))| {
            positions[i + 1..]
                .iter()
                .map(move |&&(x2, y2)| x1.abs_diff(x2) + y1.abs_diff(y2))
                .sum::<usize>()
        })
        .sum()
}

fn part1() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input file");
    let (dimensions, galaxy_positions) = parse_universe(&data);

    // println!("Original Universe:");
    // display_universe(dimensions, &galaxy_positions);

    let (scaled_dimensions, scaled_positions) =
        scale_universe(dimensions, &galaxy_positions, 2);

    // println!("Scaled Universe:");
    // display_universe(scaled_dimensions, &scaled_positions);

    let total_distance = calculate_total_manhattan_distance(&scaled_positions);
    println!("Answer: {}", total_distance);
}

fn part2() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input file");
    let (dimensions, galaxy_positions) = parse_universe(&data);

    // println!("Original Universe:");
    // display_universe(dimensions, &galaxy_positions);

    let (scaled_dimensions, scaled_positions) =
        scale_universe(dimensions, &galaxy_positions, 1_000_000);

    // println!("Scaled Universe:");
    // display_universe(scaled_dimensions, &scaled_positions);

    let total_distance = calculate_total_manhattan_distance(&scaled_positions);
    println!("Answer: {}", total_distance);
}

fn main() {
    part1();
    part2();
}
