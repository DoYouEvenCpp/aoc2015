use std::fs;
use std::collections::HashMap;

type MapType = HashMap<String, Vec<String>>;

fn parse_input(line: &str, signals: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " -> ").collect();
    let key = splitted[1].to_string();
    let entry: Vec<String> = splitted[0].splitn(3, " ")
        .map(|s| s.to_owned())
        .collect();
    signals.insert(key, entry);
}

fn get_value(s: &Vec<String>, signals: &MapType) -> u32 {
    if s.len() == 3 {
        println!("{:?}", s);
        let param1 = &s[0];
        let param2 = &s[2];
        match s[1].as_str() {
            "AND" => return get_value(param1, signals) & get_value(param2, signals),
            "OR" => return get_value(param1, signals) | get_value(param2, signals),
            "LSHIFT" => return get_value(param1, signals) << get_value(param2, signals),
            "RSHIFT" => return get_value(param1, signals) >> get_value(param2, signals),
            _ => ()
        }
    }
    else if s.len() == 2 {
        return 0;
    }
    else {
        if s[0].parse::<u32>().is_ok() {
            return s[0].parse().unwrap();
        }
        else {
            return get_value(signals.get(&s[0]).unwrap(), signals);
        }
    }
    0
}

fn part_1(input: &str) -> u32 {
    let mut map = MapType::new();
    for l in input.lines() {
        parse_input(l, &mut map);
    }
    get_value(map.get("a").unwrap(), &map)
}

fn part_2(input: &str) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day07 {
    use super::*;
    #[test]
    fn test_parse_input() {
        let mut map = MapType::new();
        parse_input("123 -> x", &mut map);
    }
}
