use std::fs;

type Map = Vec<Vec<u32>>;
type Position = (usize, usize, usize, usize);

fn pase_input(input: &str) -> Map {
    let mut x: usize = 0;
    let mut x: usize = 0;
    let mut data = Map::new();
    for line in input.lines() {
        for c in line.chars() {

        }
    }
    data
}

fn part_1(input: &Map) -> u32 {
    0
}

fn part_2(input: &Map) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = pase_input(content.trim());
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day18 {
    use super::*;
}
