use std::fs;

fn contains_vovel(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    input
        .chars()
        .fold(0, |count, c| count + vowels.contains(&c) as u32)
        >= 3
}

fn has_doubled_letter(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|chars| chars[0] == chars[1])
}

fn has_no_forbidden_strings(input: &str) -> bool {
    let forbidden  = ["ab", "cd", "pq", "xy"];
    !forbidden
        .iter()
        .any(|s| input.contains(s))
}


fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|s| contains_vovel(s) && has_doubled_letter(s) && has_no_forbidden_strings(s))
        .count()
}
fn part_2(input: &str) -> bool {
    true
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day01 {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Running test for part 1");
        has_no_forbidden_strings("DUPA");
        assert_eq!(true, part_1("ugknbfddgicrmopn"));
        assert_eq!(true, part_1("aaa"));
        assert_eq!(false, part_1("jchzalrnumimnmhp"));
        assert_eq!(false, part_1("haegwjzuvuyypxyu"));
        assert_eq!(false, part_1("dvszwmarrgswjxmb"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
    }
}
