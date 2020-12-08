// use std::collections::HashSet;
// use std::collections::VecDeque;
// use std::ops::Range;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    // OP(argument, run time)
    NOP(i64, u8),
    ACC(i64, u8),
    JMP(i64, u8),
}

fn main() -> std::io::Result<()> {
    let input1 = read_input("input1.txt").expect("An error occurred when reading input1.txt");
    let mut arr_data1 = parse_data(input1).expect("An error occurred when parsing input1.txt");

    part1(&mut arr_data1);
    // println!("--------------------------------------------------");
    // part2(&mut arr_data1);
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
            let (op_code, arg) = split_once(file_fragment, " ")
                .expect(format!("Cannot split_once {:?}", file_fragment).as_str());
            return match op_code.as_str() {
                "NOP" | "nop" => Ok(Op::NOP(parse_i64(&arg), 0)),
                "ACC" | "acc" => Ok(Op::ACC(parse_i64(&arg), 0)),
                "JMP" | "jmp" => Ok(Op::JMP(parse_i64(&arg), 0)),
                _ => panic!("This operation is not recognized"),
            };
        })
        .filter(Result::is_ok)
        .map(|op| op.expect("Should be an operation"))
        .collect();
    return Ok(parsed);
}

fn parse_i64(value: &String) -> i64 {
    return value
        .parse::<i64>()
        .expect(format!("Should be a valid i64: {:?}", value).as_str());
}

fn split_once(str: &str, pattern: &str) -> Option<(String, String)> {
    let mut split: Vec<String> = str.split(pattern).map(|s| s.to_string()).collect();
    return if split.len() >= 2 {
        let mut second = split.pop().unwrap();
        loop {
            if split.len() == 1 {
                break;
            }
            second = format!("{}{}{}", split.pop().unwrap(), pattern, second);
        }
        let first = split.pop().unwrap();
        return Some((first, second));
    } else {
        None
    };
}

fn part1(arr_data: &mut Vec<Op>) {
    let mut acc: i64 = 0;
    let mut op_pointer: i64 = 0;
    loop {
        let current_op = arr_data.get_mut(op_pointer as usize);
        if current_op.is_none() {
            break;
        }
        let current_op_unwrap = current_op.unwrap();
        match current_op_unwrap {
            Op::NOP(arg, call_time) => {
                if *call_time > 0 {
                    break;
                }
                op_pointer += 1;
                *current_op_unwrap = Op::NOP(*arg, *call_time + 1);
            }
            Op::JMP(arg, call_time) => {
                if *call_time > 0 {
                    break;
                }
                op_pointer += *arg;
                *current_op_unwrap = Op::JMP(*arg, *call_time + 1);
            }
            Op::ACC(arg, call_time) => {
                if *call_time > 0 {
                    break;
                }
                op_pointer += 1;
                acc += *arg;
                *current_op_unwrap = Op::ACC(*arg, *call_time + 1);
            }
        }
    }
    println!("Part1: {:?}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly_1() {
        let parse_result_result = parse_data(
            "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
            "
            .to_string(),
        );
        let actual = parse_result_result.unwrap();
        let expected: Vec<Op> = vec![
            Op::NOP(0, 0),
            Op::ACC(1, 0),
            Op::JMP(4, 0),
            Op::ACC(3, 0),
            Op::JMP(-3, 0),
            Op::ACC(-99, 0),
            Op::ACC(1, 0),
            Op::JMP(-4, 0),
            Op::ACC(6, 0),
        ];
        assert_eq!(actual, expected);
    }
}
