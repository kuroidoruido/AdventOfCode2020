use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpUnit {
    Operator(Operator),
    Operand(u128),
    OpenBracket,
    CloseBracket,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

type Operation = Vec<OpUnit>;

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

fn parse_data(input: String) -> Result<Vec<Operation>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| -> Operation {
            return file_fragment
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    n if n.is_numeric() => Ok(OpUnit::Operand(n.to_digit(10).unwrap() as u128)),
                    '+' => Ok(OpUnit::Operator(Operator::Add)),
                    '*' => Ok(OpUnit::Operator(Operator::Multiply)),
                    '(' => Ok(OpUnit::OpenBracket),
                    ')' => Ok(OpUnit::CloseBracket),
                    _ => Err("Invalid"),
                })
                .filter(|unit| unit.is_ok())
                .map(|unit| unit.unwrap())
                .collect();
        })
        .collect();
    return Ok(parsed);
}

fn part1(data: &Vec<Operation>) {
    let res: u128 = data.iter().map(|operation| compute(operation)).sum();
    println!("Part1: {:?}", res);
}

fn compute(operation: &Operation) -> u128 {
    fn compute_rec(op: &Operation, init_i: usize) -> (u128, usize) {
        let mut res: u128 = 0;
        let mut i: usize = init_i;
        loop {
            if i >= op.len() {
                break;
            }
            match op.get(i) {
                Some(OpUnit::Operand(operand)) => {
                    res = *operand;
                    i += 1;
                }
                Some(OpUnit::OpenBracket) => {
                    let (operand, new_i) = compute_rec(op, i + 1);
                    res = operand;
                    i = new_i;
                }
                Some(OpUnit::CloseBracket) => return (res, i + 1),
                Some(OpUnit::Operator(Operator::Add)) => {
                    let next_unit = op.get(i + 1);
                    match next_unit {
                        Some(OpUnit::Operand(operand)) => {
                            res = res + operand;
                            i += 2;
                        }
                        Some(OpUnit::OpenBracket) => {
                            let (operand, new_i) = compute_rec(op, i + 2);
                            res = res + operand;
                            i = new_i;
                        }
                        _ => panic!("Invalid OpUnit position [101]"),
                    }
                }
                Some(OpUnit::Operator(Operator::Multiply)) => {
                    let next_unit = op.get(i + 1);
                    match next_unit {
                        Some(OpUnit::Operand(operand)) => {
                            res = res * operand;
                            i += 2;
                        }
                        Some(OpUnit::OpenBracket) => {
                            let (operand, new_i) = compute_rec(op, i + 2);
                            res = res * operand;
                            i = new_i;
                        }
                        _ => panic!("Invalid OpUnit position [201]"),
                    }
                }
                _ => panic!("Invalid OpUnit position [001]"),
            }
        }
        return (res, i);
    }
    return compute_rec(operation, 0).0;
}

fn part2(data: &Vec<Operation>) {
    let res: u128 = data.iter().map(|operation| compute2(operation)).sum();
    println!("Part2: {:?}", res);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpUnit2 {
    Operand(u128),
    Multiply,
}

fn compute2(operation: &Operation) -> u128 {
    fn compute_rec(op: &Operation, init_i: usize) -> (u128, usize) {
        let mut i: usize = init_i;
        let mut op_no_add: VecDeque<OpUnit2> = VecDeque::new();
        loop {
            if i >= op.len() {
                break;
            }
            match op.get(i) {
                Some(OpUnit::Operand(operand)) => {
                    op_no_add.push_back(OpUnit2::Operand(*operand));
                    i += 1;
                }
                Some(OpUnit::OpenBracket) => {
                    let (operand, new_i) = compute_rec(op, i + 1);
                    op_no_add.push_back(OpUnit2::Operand(operand));
                    i = new_i;
                }
                Some(OpUnit::CloseBracket) => {
                    i += 1;
                    break;
                }
                Some(OpUnit::Operator(Operator::Add)) => {
                    let previous = match op_no_add.pop_back() {
                        Some(OpUnit2::Operand(operand)) => operand,
                        _ => panic!("Invalid OpUnit position [0102]"),
                    };
                    let next_unit = op.get(i + 1);
                    match next_unit {
                        Some(OpUnit::Operand(operand)) => {
                            op_no_add.push_back(OpUnit2::Operand(previous + operand));
                            i += 2;
                        }
                        Some(OpUnit::OpenBracket) => {
                            let (operand, new_i) = compute_rec(op, i + 2);
                            op_no_add.push_back(OpUnit2::Operand(previous + operand));
                            i = new_i;
                        }
                        _ => panic!("Invalid OpUnit position [0101]"),
                    }
                }
                Some(OpUnit::Operator(Operator::Multiply)) => {
                    op_no_add.push_back(OpUnit2::Multiply);
                    i += 1;
                }
                _ => panic!("Invalid OpUnit position [0001]"),
            }
        }

        let mut res: u128 = 0;
        let mut j: usize = 0;
        loop {
            if j >= op_no_add.len() {
                break;
            }
            match op_no_add.get(j) {
                Some(OpUnit2::Operand(operand)) => {
                    res = *operand;
                    j += 1;
                }
                Some(OpUnit2::Multiply) => {
                    let next_unit = op_no_add.get(j + 1);
                    match next_unit {
                        Some(OpUnit2::Operand(operand)) => {
                            res = res * operand;
                            j += 2;
                        }
                        _ => panic!("Invalid OpUnit position [1201]"),
                    }
                }
                _ => panic!("Invalid OpUnit position [1001]"),
            }
        }
        return (res, i);
    }
    return compute_rec(operation, 0).0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "2 * 3 + (4 * 5)";
        let expected = vec![vec![
            OpUnit::Operand(2),
            OpUnit::Operator(Operator::Multiply),
            OpUnit::Operand(3),
            OpUnit::Operator(Operator::Add),
            OpUnit::OpenBracket,
            OpUnit::Operand(4),
            OpUnit::Operator(Operator::Multiply),
            OpUnit::Operand(5),
            OpUnit::CloseBracket,
        ]];
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_compute_correctly_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 71);
    }

    #[test]
    fn it_should_compute_correctly_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 51);
    }

    #[test]
    fn it_should_compute_correctly_3() {
        let input = "2 * 3 + (4 * 5)";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 26);
    }

    #[test]
    fn it_should_compute_correctly_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 437);
    }

    #[test]
    fn it_should_compute_correctly_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 12240);
    }

    #[test]
    fn it_should_compute_correctly_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute(operation.last().unwrap()), 13632);
    }

    #[test]
    fn it_should_compute2_correctly_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 231);
    }

    #[test]
    fn it_should_compute2_correctly_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        println!("Operation: {}", input);
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 51);
    }

    #[test]
    fn it_should_compute2_correctly_3() {
        let input = "2 * 3 + (4 * 5)";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 46);
    }

    #[test]
    fn it_should_compute2_correctly_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 1445);
    }

    #[test]
    fn it_should_compute2_correctly_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 669060);
    }

    #[test]
    fn it_should_compute2_correctly_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let operation = parse_data(input.to_string()).unwrap();
        assert_eq!(compute2(operation.last().unwrap()), 23340);
    }
}
