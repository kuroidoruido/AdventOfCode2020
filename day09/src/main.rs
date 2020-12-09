use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let arr_data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    part1(&arr_data1);
    println!("--------------------------------------------------");
    part2(&arr_data1);
    Ok(())
}

fn read_input(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn parse_data(input: String) -> Result<Vec<u64>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty");
            }
            return Ok(file_fragment
                .parse::<u64>()
                .expect(format!("Cannot be parse as u64: {:?}", file_fragment).as_str()));
        })
        .filter(Result::is_ok)
        .map(|op| op.expect("Should be an operation"))
        .collect();
    return Ok(parsed);
}

fn part1(arr_data: &Vec<u64>) {
    println!("Part1: {:?}", find_first_invalid(arr_data));
}

fn part2(arr_data: &Vec<u64>) {
    let first_invalid: u64 = find_first_invalid(arr_data);
    let res_set: Result<VecDeque<u64>, String> =
        find_contiguous_set_summing_to(arr_data, first_invalid);
    let set = res_set.expect("Should be Ok");
    let min = set.iter().min().unwrap();
    let max = set.iter().max().unwrap();
    let sum_min_max = min + max;
    println!(
        "Part2: {:?} (min={:?}, max={:?}, min+max={:?})",
        set, min, max, sum_min_max
    );
}

fn find_first_invalid(arr_data: &Vec<u64>) -> u64 {
    let preamble_size = 25;
    let mut previous25: VecDeque<u64> = arr_data.iter().take(preamble_size).map(|u| *u).collect();
    for current in arr_data.iter().skip(preamble_size) {
        if can_add_2_to_get(&previous25, *current) {
            previous25.push_back(*current);
            previous25.pop_front();
        } else {
            return *current;
        }
    }
    return 0;
}

fn can_add_2_to_get(previous25: &VecDeque<u64>, n: u64) -> bool {
    for i in 0..25 {
        for j in 0..25 {
            if i != j {
                let one = previous25.get(i).expect(
                    format!("Should be a valid position: {:?} {:?}", previous25, i).as_str(),
                );
                let two = previous25.get(j).expect(
                    format!("Should be a valid position: {:?} {:?}", previous25, j).as_str(),
                );
                if one + two == n {
                    return true;
                }
            }
        }
    }
    return false;
}

fn find_contiguous_set_summing_to(arr_data: &Vec<u64>, n: u64) -> Result<VecDeque<u64>, String> {
    let mut current_set: VecDeque<u64> = VecDeque::new();
    for current in arr_data.iter() {
        current_set.push_back(*current);
        loop {
            let sum: u64 = current_set.iter().sum();
            if sum == n {
                return Ok(current_set);
            }
            if sum > n {
                current_set.pop_front();
            }
            if sum < n {
                break;
            }
        }
    }
    return Err("Not found...".to_string());
}
