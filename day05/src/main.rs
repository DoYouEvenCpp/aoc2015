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
    let forbidden = ["ab", "cd", "pq", "xy"];
    !forbidden.iter().any(|s| input.contains(s))
}

fn part_1(input: &str) -> usize {
    input.lines().fold(0, |sum, line| {
        sum + (contains_vovel(line) && has_doubled_letter(line) && has_no_forbidden_strings(line))
            as usize
    })
}

fn has_single_letter_repetition(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|chars| chars[0] == chars[2])
}

fn has_pair_repetition(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        //.any(|pos, ch| chars[1] != *it.peek().unwrap())
        .any(|(pos, first)| {
            input
                .chars()
                .skip(pos + 2)
                .collect::<Vec<char>>()
                .windows(2)
                .any(|second| first[0] == second[0] && first[1] == second[1])
        })
}

fn part_2(input: &str) -> usize {
    input.lines().fold(0, |sum, line| {
        sum + (has_single_letter_repetition(line) && has_pair_repetition(line))
            as usize
    })
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
        assert_eq!(1, part_1("ugknbfddgicrmopn"));
        assert_eq!(1, part_1("aaa"));
        assert_eq!(0, part_1("jchzalrnumimnmhp"));
        assert_eq!(0, part_1("haegwjzuvuyypxyu"));
        assert_eq!(0, part_1("dvszwmarrgswjxmb"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
    }

    #[test]
    fn test_has_single_letter_repetition() {
        assert_eq!(true, has_single_letter_repetition("ieodomkazucvgmuy"));
        assert_eq!(true, has_single_letter_repetition("xxyxx"));
        assert_eq!(true, has_single_letter_repetition("qjhvhtzxzqqjkmpb"));
        assert_eq!(false, has_single_letter_repetition("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_has_pair_repetition() {
        assert_eq!(true, has_pair_repetition("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, has_pair_repetition("xxyxx"));
        assert_eq!(true, has_pair_repetition("uurcxstgmygtbstg"));
        assert_eq!(false, has_pair_repetition("ieodomkazucvgmuy"));
    }
}
