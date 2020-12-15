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
    let mut base_mask: &String = &"".to_string();
    let mut memory: HashMap<u128, u128> = HashMap::new();

    for op in data.iter() {
        if let Op::Mask(mask) = op {
            base_mask = &mask;
        } else if let Op::Mem(mem_pos, value) = *op {
            let masked = apply_mask(mem_pos, base_mask);
            let unmasked = build_all_masks(&masked);
            for addr in unmasked {
                memory.insert(addr, value);
            }
        }
    }

    return memory.values().sum();
}

fn apply_mask(address: u128, mask: &String) -> String {
    let binary_address = format!("{:0>36}", format!("{:b}", address));
    let binary_array: Vec<String> = binary_address
        .chars()
        .zip(mask.chars())
        .map(|(addr_i, mask_i)| {
            if mask_i == '1' {
                return '1';
            } else if mask_i == 'X' {
                return 'X';
            } else {
                return addr_i;
            }
        })
        .map(|c| format!("{}", c))
        .collect();
    let binary_masked_address: String = binary_array.join("");
    return binary_masked_address;
}

fn build_all_masks(mask: &String) -> Vec<u128> {
    fn build_all_masks_rec(mask_rec: &String) -> Vec<u128> {
        if mask_rec.contains("X") {
            return vec![
                build_all_masks_rec(&mask_rec.replacen("X", "0", 1)),
                build_all_masks_rec(&mask_rec.replacen("X", "1", 1)),
            ]
            .iter()
            .flatten()
            .map(|m| *m)
            .collect();
        } else {
            return vec![u128::from_str_radix(mask_rec.as_str(), 2).unwrap()];
        }
    }
    return build_all_masks_rec(mask);
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
                u128::from_str_radix("000000000000000000000000000000000000", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000000001", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000000010", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000000011", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000001000", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000001001", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000001010", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000001011", 2).unwrap(),
            ]
        )
    }

    #[test]
    fn it_should_generate_all_masks_2() {
        let mask = "000000000000000000000000000000X1001X".to_string();

        assert_eq!(
            build_all_masks(&mask),
            vec![
                u128::from_str_radix("000000000000000000000000000000010010", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000010011", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000110010", 2).unwrap(),
                u128::from_str_radix("000000000000000000000000000000110011", 2).unwrap(),
            ]
        )
    }

    #[test]
    fn it_should_apply_mask_correctly_1() {
        let mem_pos: u128 = 42;
        let mask: String = "000000000000000000000000000000X1001X".to_string();
        let masked: String = apply_mask(mem_pos, &mask);
        let unmasked_mem_pos = build_all_masks(&masked);
        assert_eq!(unmasked_mem_pos, [26, 27, 58, 59]);
    }

    #[test]
    fn it_should_execute_correctly_2() {
        let input =
            "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let data = parse_data(input.to_string()).unwrap();
        assert_eq!(execute2(&data), 208);
    }
}
