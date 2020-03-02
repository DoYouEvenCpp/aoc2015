use std::fs;

type DistanceTo = (String, usize);
type Entry = (String, DistanceTo);
type MapType = std::vec::Vec<Entry>;

fn handle_entry(line: &str, map: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " = ").collect();
    let distance: usize = splitted[1].to_string().parse().unwrap();
    let towns: Vec<String> = splitted[0].splitn(2, " to ")
        .map(|s| s.to_owned())
        .collect();
    //println!("{} to {} in {}", towns[0], towns[1], distance);
    map.push((towns[0].to_owned(), (towns[1].to_owned(), distance)));
    map.push((towns[1].to_owned(), (towns[0].to_owned(), distance)));
}

fn parse_input(input: &str) -> MapType {
    let mut map = MapType::new();
    for line in input.lines() {
        handle_entry(line, &mut map);
    }
    map
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let map = parse_input(content);

    for (town, distance) in map {
        println!("{} -> {} {}", town, distance.0, distance.1);
    }
    let mut perms = map[..].permutations();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day09 {
    use super::*;
}
