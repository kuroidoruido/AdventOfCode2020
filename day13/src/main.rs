use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Infos {
    estimate_min_departure: u64,
    bus: Vec<Bus>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bus {
    bus_id: u64,
    ignored: bool,
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

fn parse_data(input: String) -> Result<Infos, String> {
    let split: Vec<&str> = input
        .split("\n")
        .map(|file_fragment| file_fragment.trim())
        .filter(|file_fragment| !file_fragment.is_empty())
        .collect();
    if split.len() != 2 {
        return Err("Wrong file format".to_string());
    }
    let estimate_min_departure = split
        .get(0)
        .unwrap()
        .parse::<u64>()
        .expect("First line should be a number");
    let bus: Vec<Bus> = split
        .get(1)
        .unwrap()
        .split(',')
        .map(|bus_id| bus_id.trim())
        .filter(|bus_id| !bus_id.is_empty())
        .map(|bus_id| {
            if bus_id == "x" {
                return Bus {
                    bus_id: 0,
                    ignored: true,
                };
            }
            return Bus {
                bus_id: bus_id.parse::<u64>().expect("Should be a number"),
                ignored: false,
            };
        })
        .collect();
    let parsed = Infos {
        estimate_min_departure,
        bus,
    };
    return Ok(parsed);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BusDeparture {
    bus_id: u64,
    next: u64,
}

fn part1(data: &Infos) {
    let bus_time_arrival: Vec<BusDeparture> = data
        .bus
        .iter()
        .filter(|bus| !bus.ignored)
        .map(|bus| {
            let mut time = 0;
            while time < data.estimate_min_departure {
                time += bus.bus_id;
            }
            return BusDeparture {
                bus_id: bus.bus_id.clone(),
                next: time,
            };
        })
        .collect();
    let next_bus = bus_time_arrival.iter().min_by_key(|bus| bus.next).unwrap();
    println!(
        "Part1: next bus: {:?}, will wait: {:?} (bus_id*waiting time={:?})",
        next_bus,
        next_bus.next - data.estimate_min_departure,
        next_bus.bus_id * (next_bus.next - data.estimate_min_departure)
    );
}

fn part2(data: &Infos) {
    let res: i128 = earliest_timestamp(data);
    println!("Part2: {:?}", res);
}

fn earliest_timestamp(data: &Infos) -> i128 {
    // here we use chinese rest theorem
    let res_mod: i128 = data
        .bus
        .iter()
        .filter(|bus| !bus.ignored)
        .map(|bus| bus.bus_id as i128)
        .product();
    let mut i: u64 = 0;
    let any_res: i128 = data
        .bus
        .iter()
        .map(|bus| {
            let index = i;
            i += 1;
            return (bus, index);
        })
        .filter(|(bus, _)| !bus.ignored)
        .map(|(bus, index)| {
            let mi = res_mod / bus.bus_id as i128;
            let (_, m_inverse, _) = bezout(mi, bus.bus_id as i128);
            let mod_t: i128 =
                (((bus.bus_id as i128) * 2 - (index as i128)) % (bus.bus_id as i128)) as i128;
            return (mi, m_inverse, mod_t);
        })
        .map(|(mi, m_inverse, mod_t)| mi * m_inverse * mod_t)
        .sum();

    return ((any_res) % res_mod + res_mod) % res_mod;
}

fn bezout(a: i128, b: i128) -> (i128, i128, i128) {
    let mut r = a;
    let mut r2 = b;
    let mut u = 1;
    let mut v = 0;
    let mut u2 = 0;
    let mut v2 = 1;
    while r2 != 0 {
        let q = r / r2;
        let rs = r;
        let us = u;
        let vs = v;
        r = r2;
        u = u2;
        v = v2;
        r2 = rs - q * r2;
        u2 = us - q * u2;
        v2 = vs - q * v2;
    }
    return (r, u, v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_compute_earliest_1() {
        let infos: Infos = parse_data("0\n7,13,x,x,59,x,31,19".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 1068781);
    }

    #[test]
    fn it_should_compute_earliest_2() {
        let infos: Infos = parse_data("0\n17,x,13,19".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 3417);
    }

    #[test]
    fn it_should_compute_earliest_3() {
        let infos: Infos = parse_data("0\n67,7,59,61".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 754018);
    }

    #[test]
    fn it_should_compute_earliest_4() {
        let infos: Infos = parse_data("0\n67,x,7,59,61".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 779210);
    }

    #[test]
    fn it_should_compute_earliest_5() {
        let infos: Infos = parse_data("0\n67,7,x,59,61".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 1261476);
    }

    #[test]
    fn it_should_compute_earliest_6() {
        let infos: Infos = parse_data("0\n1789,37,47,1889".to_string()).unwrap();
        assert_eq!(earliest_timestamp(&infos), 1202161486);
    }

    // #[test]
    // fn it_should_compute_gcd() {
    //     assert_eq!(gcd(66, 17), 8);
    // }

    #[test]
    fn it_should_compute_bezout() {
        assert_eq!(bezout(66, 17), (1, 8, -31));
        assert_eq!(bezout(102, 11), (1, 4, -37));
        assert_eq!(bezout(187, 66), (11, -1, 3));
    }
}
