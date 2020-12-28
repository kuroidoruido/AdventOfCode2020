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
    let rules_index = index_rules(data);
    let valid_message_count = data
        .messages
        .iter()
        .map(|m| match_rule(&rules_index, 0, m))
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
    // println!("index");
    // res.iter().for_each(|r| println!("\t{:?}", r));
    return res;
}

fn match_rule(rules_index: &HashMap<u32, &Rule>, rule: u32, msg: &String) -> bool {
    fn match_rule_rec(
        rules_index_rec: &HashMap<u32, &Rule>,
        msg_rec: &String,
        rule_rec: u32,
        msg_i: usize,
        depth: usize,
    ) -> (bool, usize) {
        if msg_i >= msg_rec.len() {
            println!("index out of bound {}/{}", msg_i, msg_rec);
            return (false, msg_i);
        }
        let indent: String = (0..depth)
            .into_iter()
            .map(|_| -> &str { "  " })
            .fold("".to_string(), |prev, c| format!("{}{}", prev, c));
        println!(
            "{}match_rule_rec(rule:{},msg_i:{})",
            indent, rule_rec, msg_i
        );
        return match rules_index_rec.get(&rule_rec) {
            Some(Rule::Leaf(_, c)) => {
                let res = msg_rec.chars().nth(msg_i).unwrap() == *c;
                println!(
                    "{}leaf {:?}({}) = {:?} => {}",
                    indent,
                    msg_rec.chars().nth(msg_i).unwrap(),
                    msg_i,
                    c,
                    res
                );
                return (res, msg_i + 1);
            }
            Some(Rule::Composite(_, patterns)) => {
                println!("{}any {:?}/{}", indent, patterns, msg_i);
                let mut msg_i_res = msg_i;
                let res_pattern = patterns.iter().any(|sub_pattern| {
                    println!("{}all {:?}/{}", indent, sub_pattern, msg_i);
                    let mut msg_i2 = msg_i;
                    let res_sub_pattern = sub_pattern.iter().all(|rule_ref| {
                        let (sub_pattern_res, msg_i3) = match_rule_rec(
                            rules_index_rec,
                            msg_rec,
                            rule_ref.id,
                            msg_i2,
                            depth + 1,
                        );
                        if sub_pattern_res {
                            msg_i2 = msg_i3;
                        }
                        return sub_pattern_res;
                    });
                    println!(
                        "{}all {:?}/{} => {}",
                        indent, sub_pattern, msg_i2, res_sub_pattern
                    );
                    if res_sub_pattern {
                        msg_i_res = msg_i2;
                    }
                    res_sub_pattern
                });
                println!("{}any {:?}/{} => {}", indent, patterns, msg_i, res_pattern);
                (res_pattern, msg_i_res)
            }
            _ => {
                println!("Invalid rule index: {}", rule_rec);
                return (false, msg_i);
            }
        };
    }
    let (msg_valid, msg_index) = match_rule_rec(rules_index, msg, rule, 0, 0);
    return msg_valid && msg_index >= msg.len();
}

fn part2(data: &Data) {
    let mut rules_index = index_rules(data);
    // 8: 42 | 42 8
    let rule8 = Rule::Composite(
        8,
        vec![
            vec![RuleRef::new(42)],
            vec![RuleRef::new(42), RuleRef::new(8)],
        ],
    );
    rules_index.insert(8, &rule8);
    // 11: 42 31 | 42 11 31
    let rule11 = Rule::Composite(
        11,
        vec![
            vec![RuleRef::new(42), RuleRef::new(31)],
            vec![RuleRef::new(42), RuleRef::new(11), RuleRef::new(31)],
        ],
    );
    rules_index.insert(11, &rule11);
    let valid_message_count = data
        .messages
        .iter()
        .map(|m| match_rule(&rules_index, 0, m))
        .filter(|is_matching| *is_matching)
        .count();
    println!("Part2: {:?}", valid_message_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
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
    */

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

    fn msg_should_match_rule_1(msg: &str) -> bool {
        let input = "42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1
        8: 42 | 42 8
        11: 42 31 | 42 11 31
        
        ababbb";
        let rules = parse_data(input.to_string()).unwrap();
        let rules_index = index_rules(&rules);
        return match_rule(&rules_index, 0, &msg.to_string());
    }

    #[test]
    fn it_should_match_rule_1_1() {
        assert!(msg_should_match_rule_1("bbabbbbaabaabba"));
    }
    #[test]
    fn it_should_match_rule_1_2() {
        assert!(msg_should_match_rule_1("babbbbaabbbbbabbbbbbaabaaabaaa"));
    }
    /*
    #[test]
    fn it_should_match_rule_1_3() {
        assert!(msg_should_match_rule_1(
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"
        ));
    }
    #[test]
    fn it_should_match_rule_1_4() {
        assert!(msg_should_match_rule_1("bbbbbbbaaaabbbbaaabbabaaa"));
    }
    #[test]
    fn it_should_match_rule_1_5() {
        assert!(msg_should_match_rule_1(
            "bbbababbbbaaaaaaaabbababaaababaabab"
        ));
    }
    #[test]
    fn it_should_match_rule_1_6() {
        assert!(msg_should_match_rule_1("ababaaaaaabaaab"));
    }
    #[test]
    fn it_should_match_rule_1_7() {
        assert!(msg_should_match_rule_1("ababaaaaabbbaba"));
    }
    #[test]
    fn it_should_match_rule_1_8() {
        assert!(msg_should_match_rule_1("baabbaaaabbaaaababbaababb"));
    }
    #[test]
    fn it_should_match_rule_1_9() {
        assert!(msg_should_match_rule_1("abbbbabbbbaaaababbbbbbaaaababb"));
    }
    #[test]
    fn it_should_match_rule_1_10() {
        assert!(msg_should_match_rule_1("aaaaabbaabaaaaababaa"));
    }
    #[test]
    fn it_should_match_rule_1_11() {
        assert!(msg_should_match_rule_1(
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"
        ));
    }
    #[test]
    fn it_should_match_rule_1_12() {
        assert!(msg_should_match_rule_1(
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        ));
    }

    #[test]
    fn it_should_match_rule_1_13() {
        assert!(!msg_should_match_rule_1(
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"
        ));
    }
    #[test]
    fn it_should_match_rule_1_14() {
        assert!(!msg_should_match_rule_1("aaaabbaaaabbaaa"));
    }
    #[test]
    fn it_should_match_rule_1_15() {
        assert!(!msg_should_match_rule_1("babaaabbbaaabaababbaabababaaab"));
    }
    */
}
