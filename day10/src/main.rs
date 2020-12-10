// use std::collections::VecDeque;
use std::collections::HashMap;
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
    let mut arr_data_sorted = arr_data.clone();
    arr_data_sorted.sort();

    let mut diff: Vec<u64> = Vec::new();
    let mut previous = 0;
    for i in 0..arr_data_sorted.len() {
        let current = arr_data_sorted.get(i).unwrap();
        diff.push(current - previous);
        previous = *current;
    }
    diff.push(3);

    let one_diff_count = diff.iter().filter(|n| **n == 1).count();
    let three_diff_count = diff.iter().filter(|n| **n == 3).count();

    println!(
        "Part1: 1={:?} 3={:?} 1*3={:?}",
        one_diff_count,
        three_diff_count,
        one_diff_count * three_diff_count
    );
}

fn part2(arr_data: &Vec<u64>) {
    let mut arr_data_sorted = arr_data.clone();
    arr_data_sorted.sort();

    let mut memo: HashMap<u64, u64> = HashMap::new();
    let arrangements = 1 + count_possible_arrangements(0, &arr_data_sorted, &mut memo);

    println!("Part2: {:?}", arrangements);
}

fn count_possible_arrangements(
    previous: u64,
    sorted_data: &Vec<u64>,
    memo: &mut HashMap<u64, u64>,
) -> u64 {
    if memo.contains_key(&previous) {
        return *memo.get(&previous).unwrap();
    }
    let next1 = sorted_data.get(0);
    let next2 = sorted_data.get(1);
    let next3 = sorted_data.get(2);

    if next3.is_some() && (next3.unwrap() - previous) <= 3 {
        let res = 2
            + count_possible_arrangements(*next1.unwrap(), &copy_skip(sorted_data, 1), memo)
            + count_possible_arrangements(*next2.unwrap(), &copy_skip(sorted_data, 2), memo)
            + count_possible_arrangements(*next3.unwrap(), &copy_skip(sorted_data, 3), memo);
        memo.insert(previous, res);
        return res;
    } else if next2.is_some() && (next2.unwrap() - previous) <= 3 {
        let res = 1
            + count_possible_arrangements(*next1.unwrap(), &copy_skip(sorted_data, 1), memo)
            + count_possible_arrangements(*next2.unwrap(), &copy_skip(sorted_data, 2), memo);
        memo.insert(previous, res);
        return res;
    } else if next1.is_some() && (next1.unwrap() - previous) <= 3 {
        let res = count_possible_arrangements(*next1.unwrap(), &copy_skip(sorted_data, 1), memo);
        memo.insert(previous, res);
        return res;
    } else {
        memo.insert(previous, 0);
        return 0;
    }
}

fn copy_skip(data: &Vec<u64>, skip: usize) -> Vec<u64> {
    return data.iter().skip(skip).map(|n| *n).collect();
}
