use std::fs;
use std::collections::HashMap;

type Entry = (String, String, String);
type MapType = HashMap<String, Entry>;

fn parse_input(line: &str, _signals: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " -> ").collect();
    let key = splitted[1].to_string();
    let entry: Vec<&str> = splitted[0].splitn(3, " ").collect();
    println!("{} => \t\t{:?} => \t\t{:?}", line, splitted, entry);
}

fn part_1(input: &str) -> u32 {
    let mut map = MapType::new();
    for l in input.lines() {
        parse_input(l, &mut map);
    }
    0
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
        parse_input("123 -> x", map);
    }
}
