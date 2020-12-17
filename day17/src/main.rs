use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CubeState {
    Active,
    Inactive,
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

fn parse_data(input: String) -> Result<Vec<Vec<CubeState>>, String> {
    let parsed = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| -> Vec<CubeState> {
            return s
                .chars()
                .map(|c| match c {
                    '#' => Ok(CubeState::Active),
                    '.' => Ok(CubeState::Inactive),
                    _ => Err("Invalid"),
                })
                .filter(|c| c.is_ok())
                .map(|c| c.unwrap())
                .collect();
        })
        .collect();
    return Ok(parsed);
}

fn part1(data: &Vec<Vec<CubeState>>) {
    let cubes: HashMap<(i64, i64, i64), CubeState> = init_active_cubes(data);
    let end_cubes = play_cycle(&cubes, 6);
    println!("Part1: {:?}", count_actives(&end_cubes));
}

fn init_active_cubes(data: &Vec<Vec<CubeState>>) -> HashMap<(i64, i64, i64), CubeState> {
    let mut cubes: HashMap<(i64, i64, i64), CubeState> = HashMap::new();
    for y in 0..data.len() {
        for x in 0..data.last().unwrap().len() {
            if *data.get(y).unwrap().get(x).unwrap() == CubeState::Active {
                cubes.insert((x as i64, y as i64, 0), CubeState::Active);
            }
        }
    }
    return cubes;
}

fn get_neighbors_position(base_x: i64, base_y: i64, base_z: i64) -> Vec<(i64, i64, i64)> {
    let mut neighbors: Vec<(i64, i64, i64)> = Vec::new();
    for x in (base_x - 1)..(base_x + 2) {
        for y in (base_y - 1)..(base_y + 2) {
            for z in (base_z - 1)..(base_z + 2) {
                if !(x == base_x && y == base_y && z == base_z) {
                    neighbors.push((x, y, z));
                }
            }
        }
    }
    return neighbors;
}

fn play_cycle(
    cubes: &HashMap<(i64, i64, i64), CubeState>,
    cycle_count: u32,
) -> HashMap<(i64, i64, i64), CubeState> {
    let mut current_cubes = cubes.clone();
    for _ in 0..cycle_count {
        let mut next_cubes: HashMap<(i64, i64, i64), CubeState> = HashMap::new();
        let (min_x, min_y, min_z): (i64, i64, i64) = (
            *current_cubes.iter().map(|((x, _, _), _)| x).min().unwrap(),
            *current_cubes.iter().map(|((_, y, _), _)| y).min().unwrap(),
            *current_cubes.iter().map(|((_, _, z), _)| z).min().unwrap(),
        );
        let (max_x, max_y, max_z): (i64, i64, i64) = (
            *current_cubes.iter().map(|((x, _, _), _)| x).max().unwrap(),
            *current_cubes.iter().map(|((_, y, _), _)| y).max().unwrap(),
            *current_cubes.iter().map(|((_, _, z), _)| z).max().unwrap(),
        );
        for x in (min_x - 1)..(max_x + 2) {
            for y in (min_y - 1)..(max_y + 2) {
                for z in (min_z - 1)..(max_z + 2) {
                    let state: CubeState = *current_cubes
                        .get(&(x, y, z))
                        .or(Some(&CubeState::Inactive))
                        .unwrap();
                    let neighbors = get_neighbors_position(x, y, z);
                    let actives_neighbors_count = neighbors
                        .iter()
                        .map(|(nx, ny, nz)| {
                            current_cubes
                                .get(&(*nx, *ny, *nz))
                                .or(Some(&CubeState::Inactive))
                                .unwrap()
                        })
                        .filter(|cube| **cube == CubeState::Active)
                        .count();
                    match state {
                        CubeState::Active => {
                            if actives_neighbors_count == 2 || actives_neighbors_count == 3 {
                                next_cubes.insert((x, y, z), CubeState::Active);
                            }
                        }
                        CubeState::Inactive => {
                            if actives_neighbors_count == 3 {
                                next_cubes.insert((x, y, z), CubeState::Active);
                            }
                        }
                    }
                }
            }
        }
        current_cubes = next_cubes;
    }
    return current_cubes;
}

fn part2(data: &Vec<Vec<CubeState>>) {
    let cubes: HashMap<(i64, i64, i64, i64), CubeState> = init_active_cubes4(data);
    let end_cubes = play_cycle4(&cubes, 6);
    println!("Part2: {:?}", count_actives(&end_cubes));
}

fn init_active_cubes4(data: &Vec<Vec<CubeState>>) -> HashMap<(i64, i64, i64, i64), CubeState> {
    let mut cubes: HashMap<(i64, i64, i64, i64), CubeState> = HashMap::new();
    for y in 0..data.len() {
        for x in 0..data.last().unwrap().len() {
            if *data.get(y).unwrap().get(x).unwrap() == CubeState::Active {
                cubes.insert((x as i64, y as i64, 0, 0), CubeState::Active);
            }
        }
    }
    return cubes;
}

fn get_neighbors_position4(
    base_x: i64,
    base_y: i64,
    base_z: i64,
    base_w: i64,
) -> Vec<(i64, i64, i64, i64)> {
    let mut neighbors: Vec<(i64, i64, i64, i64)> = Vec::new();
    for x in (base_x - 1)..(base_x + 2) {
        for y in (base_y - 1)..(base_y + 2) {
            for z in (base_z - 1)..(base_z + 2) {
                for w in (base_w - 1)..(base_w + 2) {
                    if !(x == base_x && y == base_y && z == base_z && w == base_w) {
                        neighbors.push((x, y, z, w));
                    }
                }
            }
        }
    }
    return neighbors;
}

fn play_cycle4(
    cubes: &HashMap<(i64, i64, i64, i64), CubeState>,
    cycle_count: u32,
) -> HashMap<(i64, i64, i64, i64), CubeState> {
    let mut current_cubes = cubes.clone();
    for _ in 0..cycle_count {
        let mut next_cubes: HashMap<(i64, i64, i64, i64), CubeState> = HashMap::new();
        let (min_x, min_y, min_z, min_w): (i64, i64, i64, i64) = (
            *current_cubes
                .iter()
                .map(|((x, _, _, _), _)| x)
                .min()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, y, _, _), _)| y)
                .min()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, _, z, _), _)| z)
                .min()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, _, _, w), _)| w)
                .min()
                .unwrap(),
        );
        let (max_x, max_y, max_z, max_w): (i64, i64, i64, i64) = (
            *current_cubes
                .iter()
                .map(|((x, _, _, _), _)| x)
                .max()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, y, _, _), _)| y)
                .max()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, _, z, _), _)| z)
                .max()
                .unwrap(),
            *current_cubes
                .iter()
                .map(|((_, _, _, w), _)| w)
                .max()
                .unwrap(),
        );
        for x in (min_x - 1)..(max_x + 2) {
            for y in (min_y - 1)..(max_y + 2) {
                for z in (min_z - 1)..(max_z + 2) {
                    for w in (min_w - 1)..(max_w + 2) {
                        let state: CubeState = *current_cubes
                            .get(&(x, y, z, w))
                            .or(Some(&CubeState::Inactive))
                            .unwrap();
                        let neighbors = get_neighbors_position4(x, y, z, w);
                        let actives_neighbors_count = neighbors
                            .iter()
                            .map(|(nx, ny, nz, nw)| {
                                current_cubes
                                    .get(&(*nx, *ny, *nz, *nw))
                                    .or(Some(&CubeState::Inactive))
                                    .unwrap()
                            })
                            .filter(|cube| **cube == CubeState::Active)
                            .count();
                        match state {
                            CubeState::Active => {
                                if actives_neighbors_count == 2 || actives_neighbors_count == 3 {
                                    next_cubes.insert((x, y, z, w), CubeState::Active);
                                }
                            }
                            CubeState::Inactive => {
                                if actives_neighbors_count == 3 {
                                    next_cubes.insert((x, y, z, w), CubeState::Active);
                                }
                            }
                        }
                    }
                }
            }
        }
        current_cubes = next_cubes;
    }
    return current_cubes;
}

fn count_actives<T>(cubes: &HashMap<T, CubeState>) -> usize {
    return cubes
        .iter()
        .map(|(_, state)| *state)
        .filter(|state| *state == CubeState::Active)
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_compute_neighbors_correctly() {
        let neighbors = vec![
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
        ];
        assert_eq!(get_neighbors_position(0, 0, 0), neighbors);
    }
}
