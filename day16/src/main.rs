use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
struct TicketsData {
    fields: Vec<TicketsDataField>,
    own_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
struct TicketsDataField {
    label: String,
    ranges: Vec<Range<u32>>,
}

type Ticket = Vec<u32>;

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

fn parse_data(input: String) -> Result<TicketsData, String> {
    let lines: Vec<String> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .map(|file_fragment| file_fragment.to_string())
        .collect();

    let mut it = lines.iter();

    let mut fields: Vec<TicketsDataField> = Vec::new();
    while let Some(line) = it.next() {
        if line == "your ticket:" {
            break;
        }
        let (label, ranges_str) = split_once(line, ":");
        let ranges: Vec<Range<u32>> = ranges_str
            .split("or")
            .map(|r| r.trim().to_string())
            .filter(|r| !r.is_empty())
            .map(|r| {
                let (rl, rr) = split_once(&r, "-");
                return (
                    rl.trim_matches(|c| c == ' ' || c == ':').to_string(),
                    rr.trim_matches(|c| c == ' ' || c == '-').to_string(),
                );
            })
            .map(|(rl, rr)| (rl.parse::<u32>().unwrap(), rr.parse::<u32>().unwrap()))
            .map(|(rl, rr)| Range {
                start: rl,
                end: rr + 1,
            })
            .collect();

        fields.push(TicketsDataField {
            label: label.to_string(),
            ranges,
        });
    }

    let own_ticket_line = it.next().unwrap();
    let own_ticket: Ticket = own_ticket_line
        .split(',')
        .map(|u| u.parse::<u32>().unwrap())
        .collect();

    it.next(); // skip 'nearby tickets:' line
    let mut nearby_tickets: Vec<Ticket> = Vec::new();
    while let Some(line) = it.next() {
        nearby_tickets.push(line.split(',').map(|u| u.parse::<u32>().unwrap()).collect());
    }

    return Ok(TicketsData {
        fields,
        own_ticket,
        nearby_tickets,
    });
}

fn split_once<'a>(s: &'a String, separator: &str) -> (&'a str, &'a str) {
    let separator_index = s.find(separator).unwrap();
    return s.split_at(separator_index);
}

fn part1(data: &TicketsData) {
    println!("Part1: {:?}", sum_invalid_ticket_value(data));
}

fn sum_invalid_ticket_value(data: &TicketsData) -> u32 {
    let rules: Vec<&Range<u32>> = data.fields.iter().map(|f| &f.ranges).flatten().collect();
    let res: u32 = data
        .nearby_tickets
        .iter()
        .map(|t| -> Vec<&u32> {
            t.iter()
                .filter(|val| !match_any_range(&rules, *val))
                .collect()
        })
        .flatten()
        .sum();
    return res;
}

fn match_any_range(ranges: &Vec<&Range<u32>>, val: &u32) -> bool {
    return ranges.iter().any(|r| r.contains(val));
}

fn part2(data: &TicketsData) {
    let valid_nearby_tickets: Vec<&Ticket> = exclude_invalid_tickets(data);
    let fields_index_on_ticket =
        compute_fields_position(&data.fields, &data.own_ticket, &valid_nearby_tickets);
    let multiply_departure_values: u128 = fields_index_on_ticket
        .iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|(_, i)| *data.own_ticket.get(*i).unwrap())
        .map(|val| val as u128)
        .product();
    println!("Part2: {}", multiply_departure_values);
}

fn exclude_invalid_tickets(data: &TicketsData) -> Vec<&Ticket> {
    let rules: Vec<&Range<u32>> = data.fields.iter().map(|f| &f.ranges).flatten().collect();
    let res: Vec<&Ticket> = data
        .nearby_tickets
        .iter()
        .filter(|t| {
            t.iter()
                .filter(|val| !match_any_range(&rules, *val))
                .count()
                == 0
        })
        .collect();
    return res;
}

fn compute_fields_position(
    fields: &Vec<TicketsDataField>,
    own_ticket: &Ticket,
    tickets: &Vec<&Ticket>,
) -> HashMap<String, usize> {
    let mut fields_indexes: HashMap<String, Vec<usize>> = HashMap::new();
    for field in fields {
        let mut eventual_indexes: Vec<usize> = Vec::new();
        for i in 0..own_ticket.len() {
            let val = own_ticket.get(i).unwrap();
            if field.ranges.iter().any(|r| r.contains(val)) {
                eventual_indexes.push(i);
            }
        }
        fields_indexes.insert(field.label.clone(), eventual_indexes);
    }
    let mut tickets_it = tickets.iter().rev();
    let mut found_field_indexes: Vec<usize> = Vec::new();
    loop {
        if fields_indexes.iter().map(|(_, v)| v.len()).sum::<usize>() == fields_indexes.len() {
            break;
        }
        if let Some(ticket) = tickets_it.next() {
            for field in fields {
                let eventual_indexes = fields_indexes.get_mut(&field.label).unwrap();
                if eventual_indexes.len() == 1 {
                    continue;
                }
                *eventual_indexes = eventual_indexes
                    .iter()
                    .filter(|i| !found_field_indexes.contains(i))
                    .filter(|i| {
                        let val = ticket.get(**i).unwrap();
                        return field.ranges.iter().any(|r| r.contains(val));
                    })
                    .map(|u| *u)
                    .collect();
                if eventual_indexes.len() == 1 {
                    found_field_indexes.push(*eventual_indexes.last().unwrap());
                }
            }
        } else {
            break;
        }
    }

    loop {
        let mut found_new = false;
        for (_, indexes) in fields_indexes.iter_mut() {
            if indexes.len() == 1 {
                continue;
            }
            *indexes = indexes
                .iter()
                .filter(|i| !found_field_indexes.contains(i))
                .map(|u| *u)
                .collect();
            if indexes.len() == 1 {
                found_new = true;
                found_field_indexes.push(*(indexes.last().unwrap()));
            }
        }

        if !found_new {
            break;
        }
    }

    return fields_indexes
        .iter()
        .map(|(key, val)| (key.clone(), *(val.last().unwrap())))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_correctly() {
        let input = "class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50
            
            your ticket:
            7,1,14
            
            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12";
        let expected = TicketsData {
            fields: vec![
                TicketsDataField {
                    label: String::from("class"),
                    ranges: vec![Range { start: 1, end: 4 }, Range { start: 5, end: 8 }],
                },
                TicketsDataField {
                    label: String::from("row"),
                    ranges: vec![Range { start: 6, end: 12 }, Range { start: 33, end: 45 }],
                },
                TicketsDataField {
                    label: String::from("seat"),
                    ranges: vec![Range { start: 13, end: 41 }, Range { start: 45, end: 51 }],
                },
            ],
            own_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };
        assert_eq!(parse_data(input.to_string()).unwrap(), expected);
    }

    #[test]
    fn it_should_sum_invalid_ticket_value() {
        let input = "class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50
            
            your ticket:
            7,1,14
            
            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12";
        let data = parse_data(input.to_string()).unwrap();
        assert_eq!(sum_invalid_ticket_value(&data), 71);
    }

    #[test]
    fn it_should_compute_fields_position() {
        let input = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19
        
        your ticket:
        11,12,13
        
        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";
        let data = parse_data(input.to_string()).unwrap();
        let valid_tickets = exclude_invalid_tickets(&data);
        let mut expected: HashMap<String, usize> = HashMap::new();
        expected.insert("class".to_string(), 1);
        expected.insert("row".to_string(), 0);
        expected.insert("seat".to_string(), 2);
        assert_eq!(
            compute_fields_position(&data.fields, &data.own_ticket, &valid_tickets),
            expected
        );
    }
}
