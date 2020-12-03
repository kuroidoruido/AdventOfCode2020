use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum MapPosition {
    OpenSquare,
    Tree,
    Invalid(char),
}

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let arr_data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    part1(&arr_data1);
    part2(&arr_data1);
    Ok(())
}

fn read_input(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn parse_data(input: String) -> Result<Vec<Vec<MapPosition>>, String> {
    let parsed = input
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        // ..#...##...###.........#..#..#.
        .map(|line| {
            let line_positions: Vec<MapPosition> = line
                .chars()
                .map(|pos| match pos {
                    '.' => MapPosition::OpenSquare,
                    '#' => MapPosition::Tree,
                    any @ _ => MapPosition::Invalid(any),
                })
                .collect();
            return line_positions;
        })
        // [OpenSquare,OpenSquare,Tree,OpenSquare,OpenSquare,OpenSquare...]
        .collect();
    return Ok(parsed);
}

fn part1(arr_data: &Vec<Vec<MapPosition>>) {
    println!("Part1: {:?}", trees_encounter(arr_data, 3, 1));
}

fn part2(arr_data: &Vec<Vec<MapPosition>>) {
    let encounters = [
        trees_encounter(arr_data, 1, 1),
        trees_encounter(arr_data, 3, 1),
        trees_encounter(arr_data, 5, 1),
        trees_encounter(arr_data, 7, 1),
        trees_encounter(arr_data, 1, 2),
    ];
    println!(
        "Part2: {:?} {:?}",
        encounters,
        encounters.iter().product::<u64>()
    );
}

fn trees_encounter(arr_data: &Vec<Vec<MapPosition>>, right_move: usize, bottom_move: usize) -> u64 {
    let mut position_x = 0;
    let encounter = arr_data
        .iter()
        .step_by(bottom_move)
        .map(|line| {
            let is_tree: u64 = match line.get(position_x) {
                Some(MapPosition::Tree) => 1,
                _ => 0,
            };
            position_x = (position_x + right_move) % line.len();
            return is_tree;
        })
        .sum();
    return encounter;
}
