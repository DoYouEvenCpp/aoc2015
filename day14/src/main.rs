use std::fs;

fn part_1(input: &str) -> i32 {
    0
}

fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();
    println!("First puzzle: {}", part_1(&input));
    println!("Second puzzle: {}", part_1(&input));
}

#[cfg(test)]
mod day13 {
    use super::*;
}
