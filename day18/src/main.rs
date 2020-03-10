use std::fs;

type Map = Vec<Vec<char>>;

fn pase_input(input: &str) -> Map {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut data = Map::new();
    for line in input.lines() {
        data.push(vec![]);
        for c in line.chars() {
            data.last_mut().unwrap().push(c);
        }
    }
    data.truncate(input.lines().count());
    data
}

fn get_number_of_neighbours(state: char, x: usize, y: usize) -> u32 {
    0
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
