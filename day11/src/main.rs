use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    part1(&data1);
    println!("--------------------------------------------------");
    part2(&data1);
    Ok(())
}

fn read_input(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn parse_data(input: String) -> Result<Vec<Vec<Position>>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty");
            }
            let row: Vec<Position> = file_fragment
                .chars()
                .map(|c| match c {
                    '.' => Position::Floor,
                    'L' => Position::EmptySeat,
                    '#' => Position::OccupiedSeat,
                    any @ _ => panic!("Invalid character: {:?}", any),
                })
                .collect();
            return Ok(row);
        })
        .filter(Result::is_ok)
        .map(|op| op.expect("Should be a position row"))
        .collect();
    return Ok(parsed);
}

fn print_state(state: &Vec<Vec<Position>>) {
    for row in state {
        let str_row: Vec<&str> = row
            .iter()
            .map(|c| match c {
                Position::Floor => ".",
                Position::EmptySeat => "L",
                Position::OccupiedSeat => "#",
            })
            .collect();
        println!("{:}", str_row.join(""));
    }
}

fn part1(data: &Vec<Vec<Position>>) {
    let mut previous: Vec<Vec<Position>> = data.clone();
    loop {
        match next_state1(&previous) {
            Some(new_state) => {
                previous = new_state;
            }
            None => break,
        }
    }
    print_state(&previous);
    println!(
        "Part1: {:?}",
        previous
            .iter()
            .flatten()
            .filter(|pos| **pos == Position::OccupiedSeat)
            .count()
    );
}

/**
 Return a new state only if at least one change was made
*/
fn next_state1(state: &Vec<Vec<Position>>) -> Option<Vec<Vec<Position>>> {
    if state.is_empty() {
        return Some(Vec::new());
    }
    let mut next: Vec<Vec<Position>> = Vec::new();
    let mut changed_something = false;

    let row_count: usize = state.len();
    let col_count: usize = state.get(0).unwrap().len();

    for row in 0..row_count {
        let mut new_row: Vec<Position> = Vec::new();
        for col in 0..col_count {
            let current: &Position = state.get(row).unwrap().get(col).unwrap();
            match current {
                Position::EmptySeat => {
                    let around = grab_position_around1(state, row, col);
                    let occupied = around
                        .iter()
                        .filter(|p| **p == Position::OccupiedSeat)
                        .count();
                    if occupied == 0 {
                        changed_something = true;
                        new_row.push(Position::OccupiedSeat);
                    } else {
                        new_row.push(Position::EmptySeat);
                    }
                }
                Position::OccupiedSeat => {
                    let around = grab_position_around1(state, row, col);
                    let occupied = around
                        .iter()
                        .filter(|p| **p == Position::OccupiedSeat)
                        .count();
                    if occupied >= 4 {
                        changed_something = true;
                        new_row.push(Position::EmptySeat);
                    } else {
                        new_row.push(Position::OccupiedSeat);
                    }
                }
                other @ _ => new_row.push(*other),
            }
        }
        next.push(new_row);
    }

    if changed_something {
        return Some(next);
    } else {
        return None;
    }
}

fn grab_position_around1(state: &Vec<Vec<Position>>, row: usize, col: usize) -> Vec<Position> {
    if state.is_empty() {
        return Vec::new();
    }
    let min_row = if row > 0 { row - 1 } else { row };
    let max_row = if row < state.len() - 1 {
        row + 2
    } else {
        row + 1
    }; // we add 1 at least because Range exclude end
    let min_col = if col > 0 { col - 1 } else { col };
    let max_col = if col < state.get(0).unwrap().len() - 1 {
        col + 2
    } else {
        col + 1
    }; // we add 1 at least because Range exclude end

    let mut res: Vec<Position> = Vec::new();
    for r in min_row..max_row {
        for c in min_col..max_col {
            if !(r == row && c == col) {
                res.push(*state.get(r).unwrap().get(c).unwrap());
            }
        }
    }
    return res;
}

fn part2(data: &Vec<Vec<Position>>) {
    let mut previous: Vec<Vec<Position>> = data.clone();
    loop {
        match next_state2(&previous) {
            Some(new_state) => {
                previous = new_state;
            }
            None => break,
        }
    }
    print_state(&previous);
    println!(
        "Part2: {:?}",
        previous
            .iter()
            .flatten()
            .filter(|pos| **pos == Position::OccupiedSeat)
            .count()
    );
}

/**
 Return a new state only if at least one change was made
*/
fn next_state2(state: &Vec<Vec<Position>>) -> Option<Vec<Vec<Position>>> {
    if state.is_empty() {
        return Some(Vec::new());
    }
    let mut next: Vec<Vec<Position>> = Vec::new();
    let mut changed_something = false;

    let row_count: usize = state.len();
    let col_count: usize = state.get(0).unwrap().len();

    for row in 0..row_count {
        let mut new_row: Vec<Position> = Vec::new();
        for col in 0..col_count {
            let current: &Position = state.get(row).unwrap().get(col).unwrap();
            match current {
                Position::EmptySeat => {
                    let around = grab_position_around2(state, row, col);
                    let occupied = around
                        .iter()
                        .filter(|p| **p == Position::OccupiedSeat)
                        .count();
                    if occupied == 0 {
                        changed_something = true;
                        new_row.push(Position::OccupiedSeat);
                    } else {
                        new_row.push(Position::EmptySeat);
                    }
                }
                Position::OccupiedSeat => {
                    let around = grab_position_around2(state, row, col);
                    let occupied = around
                        .iter()
                        .filter(|p| **p == Position::OccupiedSeat)
                        .count();
                    if occupied >= 5 {
                        changed_something = true;
                        new_row.push(Position::EmptySeat);
                    } else {
                        new_row.push(Position::OccupiedSeat);
                    }
                }
                other @ _ => new_row.push(*other),
            }
        }
        next.push(new_row);
    }

    if changed_something {
        return Some(next);
    } else {
        return None;
    }
}

fn grab_position_around2(state: &Vec<Vec<Position>>, row: usize, col: usize) -> Vec<Position> {
    if state.is_empty() {
        return Vec::new();
    }
    let mut res: Vec<Position> = Vec::new();
    let min_row = if row > 0 { row - 1 } else { row };
    let max_row = if row < state.len() - 1 {
        row + 2
    } else {
        row + 1
    }; // we add 1 at least because Range exclude end
    let min_col = if col > 0 { col - 1 } else { col };
    let max_col = if col < state.get(0).unwrap().len() - 1 {
        col + 2
    } else {
        col + 1
    }; // we add 1 at least because Range exclude end

    for r in min_row..max_row {
        for c in min_col..max_col {
            if !(r == row && c == col) {
                if let Some(pos) = grab_first_seat_in_direction(
                    state,
                    row,
                    col,
                    (r as isize) - (row as isize),
                    (c as isize) - (col as isize),
                ) {
                    res.push(pos);
                }
            }
        }
    }
    return res;
}

fn grab_first_seat_in_direction(
    state: &Vec<Vec<Position>>,
    row: usize,
    col: usize,
    row_inc: isize,
    col_inc: isize,
) -> Option<Position> {
    let mut next_r: isize = row as isize + row_inc;
    let mut next_c: isize = col as isize + col_inc;

    loop {
        if next_r < 0 || next_c < 0 {
            return None;
        }
        match state.get(next_r as usize) {
            None => return None,
            Some(row) => match row.get(next_c as usize) {
                None => return None,
                Some(pos @ Position::EmptySeat) | Some(pos @ Position::OccupiedSeat) => {
                    return Some(*pos)
                }
                _ => {
                    // Continue
                }
            },
        }
        next_r += row_inc;
        next_c += col_inc;
    }
}
