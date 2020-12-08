use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
struct BagDefinition {
    color: String,
    content: Vec<BagContent>,
    file_fragment: String,
}

#[derive(Clone, Debug, PartialEq)]
struct BagContent {
    count: usize,
    color: String,
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

fn parse_data(input: String) -> Result<Vec<BagDefinition>, String> {
    let parsed = input
        .split("\n")
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty");
            }
            // vibrant purple bags contain 3 shiny lavender bags, 1 mirrored gray bag, 4 muted bronze bags.
            // clear salmon bags contain 1 light yellow bag.
            let (bag_color, bag_content_str) = split_once(file_fragment, " bags contain")
                .expect(format!("Should contain a bag def {:?}", file_fragment).as_str());

            let content: Vec<BagContent> = bag_content_str
                // ' 3 shiny lavender bags, 1 mirrored gray bag, 4 muted bronze bags.'
                // ' 1 light yellow bag.'
                .split(|c| c == '.' || c == ',')
                // [" 3 shiny lavender bags"," 1 mirrored gray bag"," 4 muted bronze bags",""]
                // [" 1 light yellow bag",""]
                .map(|bag_content_def| bag_content_def.trim())
                .map(|bag_content_def| bag_content_def.trim_end_matches(" bag"))
                .map(|bag_content_def| bag_content_def.trim_end_matches(" bags"))
                .map(|bag_content_def| bag_content_def.trim_end_matches("no other"))
                // ["3 shiny lavender","1 mirrored gray","4 muted bronze",""]
                // ["1 light yellow bag",""]
                .filter(|bag_content_def| !bag_content_def.is_empty())
                // ["3 shiny lavender","1 mirrored gray","4 muted bronze"]
                // ["1 light yellow"]
                .map(|bag_content_def| {
                    let (count, color) = split_once(bag_content_def, " ").expect(
                        format!(
                            "Should be formatted as \"<count> <color>\" {:?}",
                            bag_content_def
                        )
                        .as_str(),
                    );
                    return BagContent {
                        count: count
                            .parse::<usize>()
                            .expect(format!("Should be a usize {:?}", count).as_str()),
                        color,
                    };
                })
                .collect();

            return Ok(BagDefinition {
                color: bag_color,
                content,
                file_fragment: file_fragment.to_string(),
            });
        })
        .filter(Result::is_ok)
        .map(|group_form| group_form.expect("Should be a bag def"))
        .collect();
    return Ok(parsed);
}

fn split_once(str: &str, pattern: &str) -> Option<(String, String)> {
    let mut split: Vec<String> = str.split(pattern).map(|s| s.to_string()).collect();
    // println!("split {:?} with {:?} give {:?}", str, pattern, split);
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

fn part1(arr_data: &Vec<BagDefinition>) {
    let mut containing_shiny_gold: HashSet<String> = HashSet::new();
    containing_shiny_gold.insert("shiny gold".to_string());
    loop {
        let before = containing_shiny_gold.len();

        arr_data.iter().for_each(|bag_def| {
            let count_bag_can_contains_shiny_gold: usize = bag_def
                .content
                .iter()
                .filter(|content| containing_shiny_gold.contains(&content.color))
                .map(|_| 1)
                .sum();
            if count_bag_can_contains_shiny_gold > 0 {
                containing_shiny_gold.insert(bag_def.color.clone());
            }
        });

        let after = containing_shiny_gold.len();
        if before == after {
            break;
        }
    }
    println!(
        "Part1: {:?}/{:?}",
        containing_shiny_gold,
        containing_shiny_gold.len()
    );
}

fn part2(arr_data: &Vec<BagDefinition>) {
    let mut result: Vec<&BagContent> = Vec::new();
    let mut pending: VecDeque<&BagContent> = VecDeque::new();
    let shiny_gold = BagContent {
        count: 1,
        color: "shiny gold".to_string(),
    };
    pending.push_back(&shiny_gold);

    loop {
        match pending.pop_front() {
            Some(current) => {
                result.push(current);
                let bag_def: &BagDefinition = arr_data
                    .iter()
                    .find(|b| b.color == current.color)
                    .expect(format!("Bag {:?} should exists.", current).as_str());
                bag_def.content.iter().for_each(|bag_content| {
                    for _ in (Range {
                        start: 0,
                        end: bag_content.count,
                    }) {
                        pending.push_back(bag_content);
                    }
                });
            }
            None => break,
        }
    }

    // we need to exclude shiny gold bag itself
    println!("Part2: {:?}", result.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly_1() {
        let parse_result_result =
            parse_data("clear aqua bags contain 2 plaid green bags.".to_string());
        let mut parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.pop();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            actual,
            BagDefinition {
                color: "clear aqua".to_string(),
                content: [BagContent {
                    count: 2,
                    color: "plaid green".to_string()
                }]
                .iter()
                .cloned()
                .collect(),
                file_fragment: "clear aqua bags contain 2 plaid green bags.".to_string(),
            }
        );
    }

    #[test]
    fn it_should_parse_correctly_2() {
        let parse_result_result = parse_data("vibrant purple bags contain 3 shiny lavender bags, 1 mirrored gray bag, 4 muted bronze bags.".to_string());
        let mut parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.pop();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            actual,
            BagDefinition {
                color: "vibrant purple".to_string(),
                content: [BagContent{ count: 3, color: "shiny lavender".to_string() },BagContent{ count: 1, color: "mirrored gray".to_string() },BagContent{ count: 4, color: "muted bronze".to_string() } ].iter().cloned().collect(),
                file_fragment: "vibrant purple bags contain 3 shiny lavender bags, 1 mirrored gray bag, 4 muted bronze bags.".to_string(),
            }
        );
    }

    #[test]
    fn it_should_parse_correctly_3() {
        let parse_result_result =
            parse_data("clear salmon bags contain 1 light yellow bag.".to_string());
        let mut parse_result = parse_result_result.unwrap();
        let first_parse_result_option = parse_result.pop();
        let actual = first_parse_result_option.unwrap();
        assert_eq!(
            actual,
            BagDefinition {
                color: "clear salmon".to_string(),
                content: [BagContent {
                    count: 1,
                    color: "light yellow".to_string()
                }]
                .iter()
                .cloned()
                .collect(),
                file_fragment: "clear salmon bags contain 1 light yellow bag.".to_string(),
            }
        );
    }
}
