use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
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

fn parse_data(input: String) -> Result<Vec<Move>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty");
            }
            let (direction, distance_str) = file_fragment.split_at(1);
            let distance = distance_str
                .parse::<i64>()
                .expect(format!("Should be a number: {:?}", distance_str).as_str());
            return match direction {
                "N" => Ok(Move::North(distance)),
                "S" => Ok(Move::South(distance)),
                "E" => Ok(Move::East(distance)),
                "W" => Ok(Move::West(distance)),
                "L" => Ok(Move::Left(distance)),
                "R" => Ok(Move::Right(distance)),
                "F" => Ok(Move::Forward(distance)),
                any @ _ => {
                    println!("Not a valid value: {:?}", any);
                    return Err("Not a valid value");
                }
            };
        })
        .filter(Result::is_ok)
        .map(|op| op.expect("Should be contain a value"))
        .collect();
    return Ok(parsed);
}

fn part1(data: &Vec<Move>) {
    let (ship_north_pos, ship_east_pos) = apply_moves1(data);
    println!(
        "Part1: {:?}+{:?}={:?}",
        ship_east_pos,
        ship_north_pos,
        ship_north_pos.abs() + ship_east_pos.abs()
    );
}

fn apply_moves1(data: &Vec<Move>) -> (i64, i64) {
    let mut ship_direction = Direction::East;
    let mut ship_north_pos: i64 = 0;
    let mut ship_east_pos: i64 = 0;

    for m in data.iter() {
        match m {
            Move::North(distance) => ship_north_pos += distance,
            Move::South(distance) => ship_north_pos -= distance,
            Move::East(distance) => ship_east_pos += distance,
            Move::West(distance) => ship_east_pos -= distance,
            Move::Forward(distance) => match ship_direction {
                Direction::North => ship_north_pos += distance,
                Direction::South => ship_north_pos -= distance,
                Direction::East => ship_east_pos += distance,
                Direction::West => ship_east_pos -= distance,
            },

            Move::Left(angle) => ship_direction = compute_next_direction(ship_direction, -angle),
            Move::Right(angle) => ship_direction = compute_next_direction(ship_direction, *angle),
        }
    }

    return (ship_north_pos, ship_east_pos);
}

fn compute_next_direction(initial: Direction, angle: i64) -> Direction {
    let fake_angle_direction = match initial {
        Direction::South => 90,
        Direction::East => 0,
        Direction::West => 180,
        Direction::North => 270,
    };
    let new_angle_direction = (fake_angle_direction + angle + 360) % 360;
    match new_angle_direction {
        0 => Direction::East,
        90 => Direction::South,
        180 => Direction::West,
        270 => Direction::North,
        any @ _ => panic!(format!("Wrong angle {:?}", any)),
    }
}

fn part2(data: &Vec<Move>) {
    let (ship_north_pos, ship_east_pos) = apply_moves2(data, 10, 1);
    println!(
        "Part2: {:?}+{:?}={:?}",
        ship_east_pos,
        ship_north_pos,
        ship_north_pos.abs() + ship_east_pos.abs()
    );
}

fn apply_moves2(data: &Vec<Move>, initial_east_speed: i64, initial_north_speed: i64) -> (i64, i64) {
    let mut ship_north_pos: i64 = 0;
    let mut ship_east_pos: i64 = 0;
    let mut east_speed = initial_east_speed;
    let mut north_speed = initial_north_speed;

    for m in data.iter() {
        match m {
            Move::North(distance) => north_speed += distance,
            Move::South(distance) => north_speed -= distance,
            Move::East(distance) => east_speed += distance,
            Move::West(distance) => east_speed -= distance,

            Move::Forward(time) => {
                ship_east_pos += time * east_speed;
                ship_north_pos += time * north_speed;
            }

            Move::Left(angle) => {
                let (new_east_speed, new_north_speed) =
                    compute_next_speed_direction(east_speed, north_speed, -angle);
                east_speed = new_east_speed;
                north_speed = new_north_speed;
            }
            Move::Right(angle) => {
                let (new_east_speed, new_north_speed) =
                    compute_next_speed_direction(east_speed, north_speed, *angle);
                east_speed = new_east_speed;
                north_speed = new_north_speed;
            }
        }
    }

    return (ship_north_pos, ship_east_pos);
}

fn compute_next_speed_direction(east_speed: i64, north_speed: i64, angle: i64) -> (i64, i64) {
    let abs_angle = (angle + 360) % 360;
    return match abs_angle {
        0 => (east_speed, north_speed),
        90 => (north_speed, -east_speed),
        180 => (-east_speed, -north_speed),
        270 => (-north_speed, east_speed),
        any @ _ => panic!(format!("Wrong angle {:?}", any)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_compute_new_angle_from_east() {
        assert_eq!(compute_next_direction(Direction::East, 0), Direction::East);
        assert_eq!(
            compute_next_direction(Direction::East, 360),
            Direction::East
        );
        assert_eq!(
            compute_next_direction(Direction::East, 90),
            Direction::South
        );
        assert_eq!(
            compute_next_direction(Direction::East, -90),
            Direction::North
        );
        assert_eq!(
            compute_next_direction(Direction::East, 180),
            Direction::West
        );
        assert_eq!(
            compute_next_direction(Direction::East, -180),
            Direction::West
        );
    }

    #[test]
    fn it_should_compute_new_angle_from_north() {
        assert_eq!(
            compute_next_direction(Direction::North, 0),
            Direction::North
        );
        assert_eq!(
            compute_next_direction(Direction::North, 360),
            Direction::North
        );
        assert_eq!(
            compute_next_direction(Direction::North, 90),
            Direction::East
        );
        assert_eq!(
            compute_next_direction(Direction::North, -90),
            Direction::West
        );
        assert_eq!(
            compute_next_direction(Direction::North, 180),
            Direction::South
        );
        assert_eq!(
            compute_next_direction(Direction::North, -180),
            Direction::South
        );
    }

    #[test]
    fn it_should_compute_next_speed_direction() {
        assert_eq!(compute_next_speed_direction(10, 4, 90), (4, -10));
        assert_eq!(compute_next_speed_direction(10, 4, 180), (-10, -4));
        assert_eq!(compute_next_speed_direction(10, 4, 270), (-4, 10));
        assert_eq!(compute_next_speed_direction(10, 4, 360), (10, 4));

        assert_eq!(compute_next_speed_direction(10, 4, -90), (-4, 10));
        assert_eq!(compute_next_speed_direction(10, 4, -180), (-10, -4));
        assert_eq!(compute_next_speed_direction(10, 4, -270), (4, -10));
        assert_eq!(compute_next_speed_direction(10, 4, -360), (10, 4));
    }
}
