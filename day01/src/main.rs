use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let input = read_input("input.txt").expect("No error when reading input.txt");

    let arr_data: Vec<u32> = input
        .split("\n")
        .map(|n| {
            n.parse::<u32>()
                .expect(format!("Should be a number: {}", n).as_str())
        })
        .collect();

    part1(&arr_data);
    part2(&arr_data);
    Ok(())
}

fn read_input(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn part1(arr_data: &Vec<u32>) {
    let mut number_that_sum_2020_1 = 0;
    let mut number_that_sum_2020_2 = 0;

    'fori: for (posi, i) in arr_data.iter().enumerate() {
        for (posj, j) in arr_data.iter().enumerate() {
            if posi != posj {
                if i + j == 2020 {
                    number_that_sum_2020_1 = *i;
                    number_that_sum_2020_2 = *j;
                    break 'fori;
                }
            }
        }
    }

    println!(
        "Part1: {:?} {:?} + = {:?} * = {:?}",
        number_that_sum_2020_1,
        number_that_sum_2020_2,
        number_that_sum_2020_1 + number_that_sum_2020_2,
        number_that_sum_2020_1 * number_that_sum_2020_2
    );
}

fn part2(arr_data: &Vec<u32>) {
    let mut number_that_sum_2020_1 = 0;
    let mut number_that_sum_2020_2 = 0;
    let mut number_that_sum_2020_3 = 0;

    'fori: for (posi, i) in arr_data.iter().enumerate() {
        for (posj, j) in arr_data.iter().enumerate() {
            if posi != posj {
                for (posk, k) in arr_data.iter().enumerate() {
                    if posi != posk && posj != posk {
                        if i + j + k == 2020 {
                            number_that_sum_2020_1 = *i;
                            number_that_sum_2020_2 = *j;
                            number_that_sum_2020_3 = *k;
                            break 'fori;
                        }
                    }
                }
            }
        }
    }

    println!(
        "Part2: {:?} {:?} {:?} + = {:?} * = {:?}",
        number_that_sum_2020_1,
        number_that_sum_2020_2,
        number_that_sum_2020_3,
        number_that_sum_2020_1 + number_that_sum_2020_2 + number_that_sum_2020_3,
        number_that_sum_2020_1 * number_that_sum_2020_2 * number_that_sum_2020_3
    );
}
