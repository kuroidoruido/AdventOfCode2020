use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct BoardingPass {
    row: usize,
    col: usize,
    seat_id: usize,
    file_fragment: String,
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

fn parse_data(input: String) -> Result<VecDeque<BoardingPass>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty boarding pass");
            }
            let (row_binary_str, col_binary_str) = file_fragment.split_at(7);
            let row_binary = row_binary_str.to_string();
            let col_binary = col_binary_str.to_string();

            let row = binary_to_number(&row_binary, 0, 127, 0);
            let col = binary_to_number(&col_binary, 0, 7, 0);
            return Ok(BoardingPass {
                row,
                col,
                seat_id: row * 8 + col,
                file_fragment: file_fragment.to_string(),
            });
        })
        .filter(Result::is_ok)
        .map(|pass| pass.expect("Should be a pass"))
        .collect();
    return Ok(parsed);
}

fn binary_to_number(binary: &String, min: usize, max: usize, index: usize) -> usize {
    if min == max {
        return min;
    }
    let half = (max - min + 1) / 2;
    return match binary.chars().nth(index) {
        Some('L') | Some('F') => binary_to_number(binary, min, max - half, index + 1),
        Some('R') | Some('B') => binary_to_number(binary, min + half, max, index + 1),
        _ => panic!("Invalid binary char: {:?}", binary),
    };
}

fn part1(arr_data: &VecDeque<BoardingPass>) {
    let highest_seat_id = arr_data.iter().map(|pass| pass.seat_id).max();
    println!("Part1: {:?}", highest_seat_id);
}

fn part2(arr_data: &VecDeque<BoardingPass>) {
    let mut all_seat_id: Vec<usize> = arr_data.iter().map(|pass| pass.seat_id).collect();
    all_seat_id.sort();

    let mut my_seat_id: Vec<usize> = Vec::new();
    for row in 0..128 {
        for col in 0..8 {
            let potential_seat_id: usize = row * 8 + col;
            if potential_seat_id > 0 && !all_seat_id.contains(&potential_seat_id) {
                if all_seat_id.contains(&(potential_seat_id - 1))
                    && all_seat_id.contains(&(potential_seat_id + 1))
                {
                    println!("Match: {:?},{:?} => {:?}", row, col, potential_seat_id);
                    my_seat_id.push(potential_seat_id);
                }
            }
        }
    }

    println!("Part2: {:?}", my_seat_id);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_should_parse_correctly_1() {
        let parse_result_result = parse_data("BFFFBBFRRR".to_string());
        let parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.front();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            *actual,
            BoardingPass {
                row: 70,
                col: 7,
                seat_id: 567,
                file_fragment: "BFFFBBFRRR".to_string(),
            }
        );
    }

    #[test]
    fn it_should_parse_correctly_2() {
        let parse_result_result = parse_data("FFFBBBFRRR".to_string());
        let parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.front();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            *actual,
            BoardingPass {
                row: 14,
                col: 7,
                seat_id: 119,
                file_fragment: "FFFBBBFRRR".to_string(),
            }
        );
    }

    #[test]
    fn it_should_parse_correctly_3() {
        let parse_result_result = parse_data("BBFFBBFRLL".to_string());
        let parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.front();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            *actual,
            BoardingPass {
                row: 102,
                col: 4,
                seat_id: 820,
                file_fragment: "BBFFBBFRLL".to_string(),
            }
        );
    }
}
