use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct CustomsDeclarationForm {
    answers: Vec<char>,
    file_fragment: String,
}

#[derive(Clone, Debug, PartialEq)]
struct GroupCustomsDeclarationForm {
    answers: Vec<CustomsDeclarationForm>,
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

fn parse_data(input: String) -> Result<Vec<GroupCustomsDeclarationForm>, String> {
    let parsed = input
        .split("\n\n")
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty group");
            }
            let mut new_group_form = GroupCustomsDeclarationForm {
                answers: Vec::new(),
                file_fragment: file_fragment.to_string(),
            };
            file_fragment
                // qzbw\nqez
                .split(|c| c == '\n' || c == ' ')
                // [ qzbw, qez ]
                .for_each(|person_answers| {
                    let new_form = CustomsDeclarationForm {
                        answers: person_answers.chars().collect(),
                        file_fragment: person_answers.to_string(),
                    };
                    new_group_form.answers.push(new_form);
                });
            return Ok(new_group_form);
        })
        .filter(Result::is_ok)
        .map(|group_form| group_form.expect("Should be a group form"))
        .collect();
    return Ok(parsed);
}

fn part1(arr_data: &Vec<GroupCustomsDeclarationForm>) {
    let result: usize = arr_data
        .iter()
        .map(|group_form| {
            group_form
                .answers
                .iter()
                .fold(HashSet::new(), |mut set, form| {
                    form.answers.iter().for_each(|question| {
                        set.insert(question);
                    });
                    return set;
                })
        })
        .map(|group_answer_set| group_answer_set.len())
        .sum();
    println!("Part1: {:?}", result);
}

fn part2(arr_data: &Vec<GroupCustomsDeclarationForm>) {
    let result: usize = arr_data
        .iter()
        .map(|group_form| {
            // grab first form answers as base result
            let mut intersec_res: HashSet<char> = group_form
                .answers
                .get(0)
                .unwrap()
                .answers
                .iter()
                .map(|c| *c)
                .collect();
            // make the intersection (= keep only common value) between intersec_res and next forms one by one
            group_form.answers.iter().skip(1).for_each(|form| {
                let current_form_answer_set: HashSet<char> =
                    form.answers.iter().map(|c| *c).collect();
                intersec_res = intersec_res
                    .intersection(&current_form_answer_set)
                    .map(|c| *c)
                    .collect();
            });
            return intersec_res;
        })
        .map(|group_answer_set| group_answer_set.len())
        .sum();
    println!("Part2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly_1() {
        let parse_result_result = parse_data("qzbw\nqez".to_string());
        let mut parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.pop();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            actual,
            CustomsDeclarationForm {
                answers: ['q', 'z', 'b', 'w', 'e', 'z'].iter().cloned().collect(), //HashSet::new(),
                file_fragment: "qzbw\nqez".to_string(),
            }
        );
    }
}
