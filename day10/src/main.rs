use std::fs;

fn single_iteration(input: &str) -> String {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut numbers: Vec<(u32, u32)> = vec![];
    let res: String = "".to_owned();
    for ch in digits {
        if let Some(e) = numbers.last_mut() {
            if e.0 == ch {
                e.1 += 1;
            }
        } else {
            numbers.push((ch, 1));
        }
    }
    println!("{:?}", numbers);
    res
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let content = "1321131112";
    single_iteration("11");
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day10 {
    use super::*;
}
