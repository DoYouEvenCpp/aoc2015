use std::fs;

fn get_total_characters_len(line: &str) -> usize {
    0
}

fn part_1(input: &str) -> u32 {
    0
}

fn part_2(_input: &str) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day08 {
    use super::*;
    #[test]
    fn test_get_total_characters_len() {
    }
}
