use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Clone, Debug)]
struct Passport {
    byr: Option<String>, // Birth Year
    iyr: Option<String>, // Issue Year
    eyr: Option<String>, // Expiration Year
    cid: Option<String>, // Country ID
    pid: Option<String>, // Passport ID
    hgt: Option<String>, // Height
    hcl: Option<String>, // Hair Color
    ecl: Option<String>, // Eye Color
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

fn parse_data(input: String) -> Result<VecDeque<Passport>, String> {
    let parsed = input
        .split("\n\n")
        .map(|file_fragment| {
            if file_fragment.is_empty() {
                return Err("Empty passport");
            }
            let mut new_passport = Passport {
                byr: None,
                iyr: None,
                eyr: None,
                cid: None,
                pid: None,
                hgt: None,
                hcl: None,
                ecl: None,
                file_fragment: file_fragment.to_string(),
            };
            file_fragment
                // ecl:grn\ncid:315 iyr:2012 hgt:192cm eyr:2023 pid:873355140 byr:1925 hcl:#cb2c03
                .split(|c| c == '\n' || c == ' ')
                // [ ecl:grn, cid:315, iyr:2012, hgt:192cm, eyr:2023, pid:873355140, byr:1925, hcl:#cb2c03 ]
                .for_each(|kv| {
                    let split: Vec<&str> = kv.split(":").collect(); // ecl:grn => [ecl, grn]
                    match split.get(0) {
                        Some(&"byr") => new_passport.byr = try_parse_as_string(&split, 1),
                        Some(&"iyr") => new_passport.iyr = try_parse_as_string(&split, 1),
                        Some(&"eyr") => new_passport.eyr = try_parse_as_string(&split, 1),
                        Some(&"cid") => new_passport.cid = try_parse_as_string(&split, 1),
                        Some(&"pid") => new_passport.pid = try_parse_as_string(&split, 1),
                        Some(&"hgt") => new_passport.hgt = try_parse_as_string(&split, 1),
                        Some(&"hcl") => new_passport.hcl = try_parse_as_string(&split, 1),
                        Some(&"ecl") => new_passport.ecl = try_parse_as_string(&split, 1),
                        Some(_) => panic!("Should not be detect as a key-value: {:?}", kv),
                        _ => panic!(
                            "Should not get an empty key-value: {:?} {:?}",
                            kv, file_fragment
                        ),
                    }
                });
            return Ok(new_passport);
        })
        .filter(Result::is_ok)
        .map(|passport| passport.expect("Should be a passport"))
        .collect();
    return Ok(parsed);
}

fn try_parse_as_string(kv: &Vec<&str>, index: usize) -> Option<String> {
    return match kv.get(index) {
        Some(str_value) => Some(str_value.to_string()),
        _ => {
            println!("No value for this kv {:?} {:?}", kv, 1);
            return None;
        }
    };
}

fn part1(arr_data: &VecDeque<Passport>) {
    let valid_count: usize = arr_data
        .iter()
        .filter(|passport| {
            let is_valid = passport.byr.is_some()
                && passport.iyr.is_some()
                && passport.eyr.is_some()
                && passport.pid.is_some()
                && passport.hgt.is_some()
                && passport.hcl.is_some()
                && passport.ecl.is_some();
            // if !is_valid {
            //     println!("invalid passport: {:#?}", passport);
            // }
            return is_valid;
        })
        .count();
    println!("Part1: {:?}/{:?}", valid_count, arr_data.len());
}

fn part2(arr_data: &VecDeque<Passport>) {
    let valid_count: usize = arr_data
        .iter()
        .filter(|passport| {
            let is_valid = passport.byr.is_some()
                && valid_number_range(&passport.byr.as_ref().unwrap(), 1920, 2002)
                // issue year
                && passport.iyr.is_some()
                && valid_number_range(&passport.iyr.as_ref().unwrap(), 2010, 2020)
                // expiration year
                && passport.eyr.is_some()
                && valid_number_range(&passport.eyr.as_ref().unwrap(), 2020, 2030)
                // passport ID
                && passport.pid.is_some()
                && valid_passport_id(&passport.pid.as_ref().unwrap())
                // height
                && passport.hgt.is_some()
                && valid_height(&passport.hgt.as_ref().unwrap())
                // hair color
                && passport.hcl.is_some()
                && valid_hair_color(&passport.hcl.as_ref().unwrap())
                // eye color
                && passport.ecl.is_some()
                && valid_eye_color(&passport.ecl.as_ref().unwrap());
            // if !is_valid {
            //     println!("invalid passport: {:#?}", passport);
            // }
            return is_valid;
        })
        .count();
    println!("Part2: {:?}/{:?}", valid_count, arr_data.len());
}

fn valid_number_range(value: &String, min: u64, max: u64) -> bool {
    let res = match value.parse::<u64>() {
        Ok(n) => min <= n && n <= max,
        _ => false,
    };
    // if !res {
    //     println!("Invalid number: {:?} <= {:?} <= {:?}", min, value, max);
    // }
    return res;
}

fn valid_passport_id(value: &String) -> bool {
    let passport_id_regex: Regex = Regex::new(r"^\d{9}$").unwrap();
    let res = passport_id_regex.is_match(value);
    // if !res {
    //     println!("Invalid passport id: {:?}", value);
    // }
    return res;
}

fn valid_height(value: &String) -> bool {
    let res = if value.ends_with("cm") {
        let value_without_suffix = value.trim_end_matches("cm");
        match value_without_suffix.parse::<u64>() {
            Ok(n) => 150 <= n && n <= 193,
            _ => false,
        }
    } else if value.ends_with("in") {
        let value_without_suffix = value.trim_end_matches("in");
        match value_without_suffix.parse::<u64>() {
            Ok(n) => 59 <= n && n <= 76,
            _ => false,
        }
    } else {
        false
    };
    // if !res {
    //     println!("Invalid height: {:?}", value);
    // }
    return res;
}

fn valid_hair_color(value: &String) -> bool {
    let hair_color_regex: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let res = hair_color_regex.is_match(value);
    // if !res {
    //     println!("Invalid hair color: {:?}", value);
    // }
    return res;
}

static EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn valid_eye_color(value: &String) -> bool {
    let res = EYE_COLORS.contains(&value.as_str());
    // if !res {
    //     println!("Invalid eye color: {:?}", value);
    // }
    return res;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_should_valid_hair_color1() {
        assert_eq!(valid_hair_color(&"#c0946f".to_string()), true);
    }
}
