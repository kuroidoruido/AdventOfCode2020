use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Mask(String),
    Mem(u128, u128),
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

fn parse_data(input: String) -> Result<Vec<Op>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty");
            }
            if file_fragment.starts_with("mask") {
                // mask = 0X11XX1X010X01101000X01X011101100000
                return Ok(Op::Mask(
                    file_fragment.trim_start_matches("mask = ").to_string(),
                ));
            } else {
                // mem[4634] = 907
                let infos: Vec<&str> = file_fragment
                    .split(|c| c == '[' || c == ']' || c == '=')
                    .map(|s| s.trim_start_matches("mem"))
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                if infos.len() != 2 {
                    println!("Invalid file_fragment: {:?} => {:?}", file_fragment, infos);
                    return Err("Invalid file_fragment");
                }
                let address = infos.get(0).unwrap().parse::<u128>().unwrap();
                let value = infos.get(1).unwrap().parse::<u128>().unwrap();
                return Ok(Op::Mem(address, value));
            }
        })
        .filter(Result::is_ok)
        .map(|op| op.expect("Should be contain a value"))
        .collect();
    return Ok(parsed);
}

fn part1(data: &Vec<Op>) {
    println!("Part1: {:?}", execute1(data));
}

fn execute1(data: &Vec<Op>) -> u128 {
    let mut zero_mask: u128 = 0;
    let mut one_mask: u128 = 0;
    let mut memory: HashMap<u128, u128> = HashMap::new();

    for op in data.iter() {
        if let Op::Mask(mask) = op {
            zero_mask = u128::from_str_radix(mask.replace('X', "1").as_str(), 2).unwrap();
            one_mask = u128::from_str_radix(mask.replace('X', "0").as_str(), 2).unwrap();
        } else if let Op::Mem(mem_pos, value) = *op {
            memory.insert(mem_pos, (value | one_mask) & zero_mask);
        }
    }

    return memory.values().sum();
}

fn part2(data: &Vec<Op>) {
    println!("Part2: {:?}", execute2(data));
}

fn execute2(data: &Vec<Op>) -> u128 {
    let mut base_mask: u128 = 0;
    let mut masks: Vec<u128> = Vec::new();
    let mut memory: HashMap<u128, u128> = HashMap::new();

    for op in data.iter() {
        if let Op::Mask(mask) = op {
            base_mask = u128::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap();
            masks = build_all_masks(mask);
        } else if let Op::Mem(mem_pos, value) = *op {
            for mask in &masks {
                memory.insert((mem_pos | base_mask | mask) & mask, value);
            }
        }
    }

    return memory.values().sum();
}

fn build_all_masks(mask: &String) -> Vec<u128> {
    fn build_all_masks_rec(mask_rec: &String) -> Vec<u128> {
        if mask_rec.contains("X") {
            return vec![
                build_all_masks_rec(&mask_rec.replacen("X", "1", 1)),
                build_all_masks_rec(&mask_rec.replacen("X", "0", 1)),
            ]
            .iter()
            .flatten()
            .map(|m| *m)
            .collect();
        } else {
            return vec![u128::from_str_radix(mask_rec.as_str(), 2).unwrap()];
        }
    }
    return build_all_masks_rec(&mask.replace("0", "1"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let expected = vec![
            Op::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
            Op::Mem(8, 11),
            Op::Mem(7, 101),
            Op::Mem(8, 0),
        ];
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_execute_correctly_1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let data = parse_data(input.to_string()).unwrap();
        assert_eq!(execute1(&data), 165);
    }

    #[test]
    fn it_should_generate_all_masks_1() {
        let mask = "00000000000000000000000000000000X0XX".to_string();

        assert_eq!(
            build_all_masks(&mask),
            vec![
                u128::from_str_radix("111111111111111111111111111111111111", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111111110", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111111101", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111111100", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111110111", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111110110", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111110101", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111110100", 2).unwrap(),
            ]
        )
    }

    #[test]
    fn it_should_generate_all_masks_2() {
        let mask = "000000000000000000000000000000X1001X".to_string();

        assert_eq!(
            build_all_masks(&mask),
            vec![
                u128::from_str_radix("111111111111111111111111111111111111", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111111110", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111011111", 2).unwrap(),
                u128::from_str_radix("111111111111111111111111111111011110", 2).unwrap(),
            ]
        )
    }

    #[test]
    fn it_should_execute_correctly_2() {
        let input =
            "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let data = parse_data(input.to_string()).unwrap();
        assert_eq!(execute2(&data), 208);
    }
}
