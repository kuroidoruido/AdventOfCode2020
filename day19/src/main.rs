use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Data {
    rules: Vec<Rule>,
    messages: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum Rule {
    Leaf(u32, char),                   // 1: "a" => Leaf(1, 'a')
    Composite(u32, Vec<Vec<RuleRef>>), // 4: 1 2 | 2 1 => Composite(4, [[Ref(1),Ref(2)],[Ref(2),Ref(1)]])
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct RuleRef {
    id: u32,
}
impl RuleRef {
    #[inline]
    fn new(id: u32) -> RuleRef {
        return RuleRef { id };
    }
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

fn parse_data(input: String) -> Result<Data, String> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut messages: Vec<String> = Vec::new();

    let mut lines = input.split("\n");

    while let Some(raw_line) = lines.next() {
        let line: &str = raw_line.trim();
        if line.is_empty() {
            break;
        }
        let colon_index: usize = line.find(":").unwrap();
        let (rule_id_str, rest) = line.split_at(colon_index);
        let rule_id: u32 = rule_id_str.parse::<u32>().unwrap();

        if rest.contains('"') {
            rules.push(Rule::Leaf(
                rule_id,
                rest.trim_start_matches(":")
                    .trim()
                    .trim_matches('"')
                    .chars()
                    .last()
                    .unwrap(),
            ));
        } else {
            let patterns: Vec<Vec<RuleRef>> = rest
                .trim_start_matches(":")
                .split("|")
                .map(|sub_pattern| -> Vec<RuleRef> {
                    return sub_pattern
                        .split(" ")
                        .map(|fragment| fragment.trim())
                        .filter(|fragment| !fragment.is_empty())
                        .map(|fragment| RuleRef::new(fragment.parse::<u32>().unwrap()))
                        .collect();
                })
                .collect();
            rules.push(Rule::Composite(rule_id, patterns.clone()));
        }
    }
    while let Some(raw_line) = lines.next() {
        let line: &str = raw_line.trim();
        if line.is_empty() {
            break;
        }
        messages.push(line.to_string());
    }

    return Ok(Data { rules, messages });
}

fn part1(data: &Data) {
    let rule_index = index_rules(data);
    let valid_message_count = data
        .messages
        .iter()
        .map(|m| match_rule(&rule_index, 0, m))
        .filter(|is_matching| *is_matching)
        .count();
    println!("Part1: {:?}", valid_message_count);
}

fn index_rules(data: &Data) -> HashMap<u32, &Rule> {
    let mut res: HashMap<u32, &Rule> = HashMap::new();
    for r in &data.rules {
        let id = match r {
            Rule::Leaf(id, _) => *id,
            Rule::Composite(id, _) => *id,
        };
        res.insert(id, r);
    }
    return res;
}

fn match_rule(rules_index: &HashMap<u32, &Rule>, rule: u32, msg: &String) -> bool {
    fn match_rule_rec(
        rules_index_rec: &HashMap<u32, &Rule>,
        msg_rec: &String,
        rule_rec: u32,
        msg_i: &mut usize,
    ) -> bool {
        return match rules_index_rec.get(&rule_rec) {
            Some(Rule::Leaf(_, c)) => msg_rec.chars().nth(*msg_i).unwrap() == *c,
            Some(Rule::Composite(_, patterns)) => patterns.iter().any(|sub_pattern| {
                let mut msg_i2 = msg_i.clone();
                return sub_pattern.iter().all(|rule_ref| {
                    match_rule_rec(rules_index_rec, msg_rec, rule_ref.id, &mut msg_i2)
                });
            }),
            _ => {
                println!("Invalid rule index: {}", rule_rec);
                return false;
            }
        };
    }
    let mut msg_index: usize = 0;
    return match_rule_rec(rules_index, msg, rule, &mut msg_index);
}

fn part2(data: &Data) {
    println!("Part2: {:?}", data.messages.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb";
        let expected = Data {
            rules: vec![
                Rule::Composite(
                    0,
                    vec![vec![RuleRef::new(4), RuleRef::new(1), RuleRef::new(5)]],
                ),
                Rule::Composite(
                    1,
                    vec![
                        vec![RuleRef::new(2), RuleRef::new(3)],
                        vec![RuleRef::new(3), RuleRef::new(2)],
                    ],
                ),
                Rule::Composite(
                    2,
                    vec![
                        vec![RuleRef::new(4), RuleRef::new(4)],
                        vec![RuleRef::new(5), RuleRef::new(5)],
                    ],
                ),
                Rule::Composite(
                    3,
                    vec![
                        vec![RuleRef::new(4), RuleRef::new(5)],
                        vec![RuleRef::new(5), RuleRef::new(4)],
                    ],
                ),
                Rule::Leaf(4, 'a'),
                Rule::Leaf(5, 'b'),
            ],
            messages: vec![
                "ababbb".to_string(),
                "bababa".to_string(),
                "abbbab".to_string(),
                "aaabbb".to_string(),
                "aaaabbb".to_string(),
            ],
        };
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_index_rules_correctly() {
        let input = "0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb";
        let rules = parse_data(input.to_string()).unwrap();
        let mut expected: HashMap<u32, &Rule> = HashMap::new();
        let r0 = Rule::Composite(
            0,
            vec![vec![RuleRef::new(4), RuleRef::new(1), RuleRef::new(5)]],
        );
        let r1 = Rule::Composite(
            1,
            vec![
                vec![RuleRef::new(2), RuleRef::new(3)],
                vec![RuleRef::new(3), RuleRef::new(2)],
            ],
        );
        let r2 = Rule::Composite(
            2,
            vec![
                vec![RuleRef::new(4), RuleRef::new(4)],
                vec![RuleRef::new(5), RuleRef::new(5)],
            ],
        );
        let r3 = Rule::Composite(
            3,
            vec![
                vec![RuleRef::new(4), RuleRef::new(5)],
                vec![RuleRef::new(5), RuleRef::new(4)],
            ],
        );
        let r4 = Rule::Leaf(4, 'a');
        let r5 = Rule::Leaf(5, 'b');
        expected.insert(0, &r0);
        expected.insert(1, &r1);
        expected.insert(2, &r2);
        expected.insert(3, &r3);
        expected.insert(4, &r4);
        expected.insert(5, &r5);

        assert_eq!(index_rules(&rules), expected);
    }

    fn msg_should_match_rule_0(msg: &str) -> bool {
        let input = "0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"
        
        ababbb";
        let rules = parse_data(input.to_string()).unwrap();
        let rules_index = index_rules(&rules);
        return match_rule(&rules_index, 0, &msg.to_string());
    }

    #[test]
    fn it_should_match_rule_1() {
        assert!(msg_should_match_rule_0("ababbb"));
    }

    #[test]
    fn it_should_match_rule_2() {
        assert!(msg_should_match_rule_0("abbbab"));
    }

    #[test]
    fn it_should_match_rule_3() {
        assert!(!msg_should_match_rule_0("bababa"));
    }

    #[test]
    fn it_should_match_rule_4() {
        assert!(!msg_should_match_rule_0("aaabbb"));
    }

    #[test]
    fn it_should_match_rule_5() {
        assert!(!msg_should_match_rule_0("aaaabbb"));
    }

    // #[test]
    // fn it_should_match_rule_2() {
    //     let input = "0: 4 1 5
    //     1: 2 3 | 3 2
    //     2: 4 4 | 5 5
    //     3: 4 5 | 5 4
    //     4: \"a\"
    //     5: \"b\"

    //     ababbb
    //     bababa
    //     abbbab
    //     aaabbb
    //     aaaabbb";
    //     let rules = parse_data(input.to_string()).unwrap();
    //     let rules_index = index_rules(&rules);

    //     assert!(!match_rule(&rules_index, 0, &"bababa".to_string()));
    //     assert!(!match_rule(&rules_index, 0, &"aaabbb".to_string()));
    //     assert!(!match_rule(&rules_index, 0, &"aaaabbb".to_string()));
    // }
}
