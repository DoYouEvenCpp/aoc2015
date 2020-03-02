use std::fs;


fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day09 {
    use super::*;
}
