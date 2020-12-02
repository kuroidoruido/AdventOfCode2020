use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct PasswordLine {
    min: usize,
    max: usize,
    character: String,
    password: String,
}

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let arr_data1 = parse_data(input1);

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

fn parse_data(input: String) -> Vec<PasswordLine> {
    let parsed = input
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        // 17-20 x: zsxjrxkgxxxxxxxmxgxf
        .map(|line| {
            let line_without_caret = line.replace('-', " ");
            let line_without_double_dot = line_without_caret.replace(": ", " ");
            let split: Vec<&str> = line_without_double_dot.split(' ').collect();
            // [17, 20, x, zsxjrxkgxxxxxxxmxgxf]
            let min_str = split
                .get(0)
                .expect(format!("Cannot get index 0 {:?}", split).as_str());
            let max_str = split
                .get(1)
                .expect(format!("Cannot get index 1 {:?}", split).as_str());
            let character = split
                .get(2)
                .expect(format!("Cannot get index 2 {:?}", split).as_str());
            let password = split
                .get(3)
                .expect(format!("Cannot get index 3 {:?}", split).as_str());
            return PasswordLine {
                min: min_str
                    .parse::<usize>()
                    .expect(format!("Cannot parse as usize '{:?}'", min_str).as_str()),
                max: max_str
                    .parse::<usize>()
                    .expect(format!("Cannot parse as usize '{:?}'", max_str).as_str()),
                character: character.to_string(),
                password: password.to_string(),
            };
        })
        .collect();
    return parsed;
}

fn part1(arr_data: &Vec<PasswordLine>) {
    let valid_ones: Vec<&PasswordLine> = arr_data
        .iter()
        .filter(|line| {
            let matches: Vec<&str> = line.password.matches(&line.character).collect();
            let match_count = matches.len();
            let is_valid: bool = line.min <= match_count && match_count <= line.max;
            // if !is_valid {
            //     println!("Invalid: {:?} (matches: {:?})", line, match_count);
            // }
            return is_valid;
        })
        .collect();

    println!("Part1: {:?}", valid_ones.len());
}

fn part2(arr_data: &Vec<PasswordLine>) {
    let valid_ones: Vec<&PasswordLine> = arr_data
        .iter()
        .filter(|line| {
            let first = line
                .password
                .chars()
                .nth(line.min - 1)
                .expect(format!("Cannot get char {:?}[{:?}]", line, line.min).as_str());
            let second = line
                .password
                .chars()
                .nth(line.max - 1)
                .expect(format!("Cannot get char {:?}[{:?}]", line, line.max).as_str());
            // ^ = XOR
            let is_valid: bool =
                (line.character.contains(first)) ^ (line.character.contains(second));
            // if !is_valid {
            //     println!(
            //         "Invalid: {:?} (extracted: {:?}:{:?} {:?}:{:?})",
            //         line, line.min, first, line.max, second
            //     );
            // }
            return is_valid;
        })
        .collect();

    println!("Part2: {:?}", valid_ones.len());
}
