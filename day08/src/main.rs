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

fn part1(arr_data: &Vec<Op>) {
    let mut arr_data_clone: Vec<Op> = arr_data.clone();
    let res: ExecuteResult = execute(&mut arr_data_clone);
    println!("Part1: {:?}", res);
}

fn part2(arr_data: &Vec<Op>) {
    let mut op_pointer: i64 = 0;
    loop {
        let current_op = arr_data.get(op_pointer as usize);
        match current_op {
            Some(Op::NOP(arg, call_time)) => {
                let mut arr_data_2: Vec<Op> =
                    fix_op(arr_data, op_pointer, Op::JMP(*arg, *call_time));
                match execute(&mut arr_data_2) {
                    ExecuteResult::EndOfProgram(res) => {
                        println!("Part2: {:?}", res);
                        return;
                    }
                    ExecuteResult::LoopingEnd(_) => {
                        // println!("Fail to fix program permuting NOP->JMP ({:?})", current_op);
                        op_pointer += 1;
                    }
                }
            }
            Some(Op::JMP(arg, call_time)) => {
                let mut arr_data_2: Vec<Op> =
                    fix_op(arr_data, op_pointer, Op::NOP(*arg, *call_time));
                match execute(&mut arr_data_2) {
                    ExecuteResult::EndOfProgram(res) => {
                        println!("Part2: {:?}", res);
                        return;
                    }
                    ExecuteResult::LoopingEnd(_) => {
                        // println!("Fail to fix program permuting JMP->NOP ({:?})", current_op);
                        op_pointer += 1;
                    }
                }
            }
            Some(_) => {
                op_pointer += 1;
            }
            None => {
                println!("Part2: no result found");
                return;
            }
        }
    }
}

fn fix_op(arr_data: &Vec<Op>, op_pointer: i64, new_op: Op) -> Vec<Op> {
    let mut arr_data_clone: Vec<Op> = arr_data.clone();
    if let Some(op) = arr_data_clone.get_mut(op_pointer as usize) {
        *op = new_op;
    }
    return arr_data_clone;
}

#[derive(Debug)]
enum ExecuteResult {
    EndOfProgram(i64),
    LoopingEnd(i64),
}

fn execute(arr_data: &mut Vec<Op>) -> ExecuteResult {
    let mut acc: i64 = 0;
    let mut op_pointer: i64 = 0;
    loop {
        let current_op = arr_data.get_mut(op_pointer as usize);
        if current_op.is_none() {
            return ExecuteResult::EndOfProgram(acc);
        }
        let current_op_unwrap = current_op.unwrap();
        match current_op_unwrap {
            Op::NOP(arg, call_time) => {
                if *call_time > 0 {
                    return ExecuteResult::LoopingEnd(acc);
                }
                op_pointer += 1;
                *current_op_unwrap = Op::NOP(*arg, *call_time + 1);
            }
            Op::JMP(arg, call_time) => {
                if *call_time > 0 {
                    return ExecuteResult::LoopingEnd(acc);
                }
                op_pointer += *arg;
                *current_op_unwrap = Op::JMP(*arg, *call_time + 1);
            }
            Op::ACC(arg, call_time) => {
                if *call_time > 0 {
                    return ExecuteResult::LoopingEnd(acc);
                }
                op_pointer += 1;
                acc += *arg;
                *current_op_unwrap = Op::ACC(*arg, *call_time + 1);
            }
        }
    }
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
