use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let data1 = parse_data("13,16,0,12,15,1").expect("An error occurred when parsing input1.txt");

    part1(&data1);
    println!("--------------------------------------------------");
    part2(&data1);
    Ok(())
}

fn parse_data(input: &str) -> Result<Vec<u128>, String> {
    let parsed = input
        .split(",")
        .map(|n| n.parse::<u128>().unwrap())
        .collect();
    return Ok(parsed);
}

fn part1(data: &Vec<u128>) {
    println!("Part1: {:?}", run_to_nth(data, 2020));
}

fn run_to_nth(data: &Vec<u128>, nth: usize) -> u128 {
    let mut last_pos: HashMap<u128, (u128, u128)> = HashMap::new();
    for i in 0..data.len() {
        last_pos.insert(*data.get(i).unwrap(), (i as u128, i as u128));
    }
    let mut all_numbers = data.clone();
    while all_numbers.len() < nth {
        let previous = all_numbers.last().unwrap();
        let (previous_previous_pos, previous_pos) = last_pos.get(previous).unwrap();

        let new_number = if previous_previous_pos == previous_pos {
            0
        } else {
            previous_pos - previous_previous_pos
        };
        let new_pos = all_numbers.len() as u128;
        all_numbers.push(new_number);

        if last_pos.contains_key(&new_number) {
            let (_, new_number_previous_pos) = last_pos.get(&new_number).unwrap();
            let new_number_previous_pos_clone = new_number_previous_pos.clone();
            last_pos.insert(new_number, (new_number_previous_pos_clone, new_pos));
        } else {
            last_pos.insert(new_number, (new_pos, new_pos));
        }
    }
    return *all_numbers.last().unwrap();
}

fn part2(data: &Vec<u128>) {
    println!("Part2: {:?}", run_to_nth(data, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        assert_eq!(
            parse_data("13,16,0,12,15,1").unwrap(),
            vec![13, 16, 0, 12, 15, 1]
        );
    }

    #[test]
    fn it_should_get_2020th_of_0_3_6() {
        assert_eq!(run_to_nth(&vec![0, 3, 6]), 436);
    }

    #[test]
    fn it_should_get_2020th_of_1_3_2() {
        assert_eq!(run_to_nth(&vec![1, 3, 2], 2020), 1);
    }

    #[test]
    fn it_should_get_2020th_of_2_1_3() {
        assert_eq!(run_to_nth(&vec![2, 1, 3], 2020), 10);
    }

    #[test]
    fn it_should_get_2020th_of_1_2_3() {
        assert_eq!(run_to_nth(&vec![1, 2, 3], 2020), 27);
    }

    #[test]
    fn it_should_get_2020th_of_2_3_1() {
        assert_eq!(run_to_nth(&vec![2, 3, 1], 2020), 78);
    }

    #[test]
    fn it_should_get_2020th_of_3_2_1() {
        assert_eq!(run_to_nth(&vec![3, 2, 1], 2020), 438);
    }

    #[test]
    fn it_should_get_2020th_of_3_1_2() {
        assert_eq!(run_to_nth(&vec![3, 1, 2], 2020), 1836);
    }
}
