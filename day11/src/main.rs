use std::convert::TryInto;

fn contains_forbidden_letters(input: &str) -> bool {
    let vowels = ['i', 'u', 'l'];
    input
        .chars()
        .fold(0, |count, c| count + vowels.contains(&c) as u32)
        > 0
}

fn contains_increasing_three_letters(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|chars| chars[1] - chars[0] == 1 && chars[2] - chars[1] == 1)
}

fn has_pair_repetition(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .any(|(pos, first)| {
            input
                .chars()
                .skip(pos + 2)
                .collect::<Vec<char>>()
                .windows(2)
                .any(|second| {
                    first[0] == first[1] && second[0] == second[1] && first[0] != second[0]
                })
        })
}
fn validate_password(input: &str) -> bool {
    !contains_forbidden_letters(input)
        && contains_increasing_three_letters(input)
        && has_pair_repetition(input)
}

fn generate_next_password(mut input: [u8; 8]) -> String {
    use std::str;
    while !validate_password(std::str::from_utf8(&input).unwrap()) {
        input[7] += 1;
        for i in (0..8).rev() {
            if input[i] > b'z' {
                input[i] = b'a';
                input[i - 1] += 1;
            }
            if validate_password(std::str::from_utf8(&input).unwrap()) {
                break;
            }
        }
    }
    std::str::from_utf8(&input).unwrap().to_string()
}

fn part_1(input: &str) -> String {
    let mut input: [u8; 8] = input.as_bytes().try_into().unwrap();
    generate_next_password(input)
}

fn main() {
    let content = "hepxcrrq";
    println!("First puzzle: {}", part_1(content));
    println!("Second puzzle: {}", part_1("hepxxzaa"));
}

#[cfg(test)]
mod day11 {
    use super::*;
    #[test]
    fn test_contains_increasing_three_letters() {
        assert_eq!(true, contains_increasing_three_letters("hijklmmn"));
        assert_eq!(false, contains_increasing_three_letters("abbceffg"));
        assert_eq!(false, contains_increasing_three_letters("abbcegjk"));
        assert_eq!(false, contains_increasing_three_letters(""));
    }

    #[test]
    fn test_contains_forbidden_letters() {
        assert_eq!(true, contains_forbidden_letters("hijklmmn"));
        assert_eq!(false, contains_forbidden_letters("abbceffg"));
        assert_eq!(false, contains_forbidden_letters("abbcegjk"));
        assert_eq!(true, contains_forbidden_letters("iol"));
        assert_eq!(false, contains_forbidden_letters(""));
    }
    #[test]
    fn test_has_pair_repetition() {
        assert_eq!(true, has_pair_repetition("abbceffg"));
        assert_eq!(false, has_pair_repetition("abbcegjk"));
        assert_eq!(false, has_pair_repetition("aaa"));
    }
    #[test]
    fn test_validate_password() {
        assert_eq!(true, validate_password("abcdffaa"));
        assert_eq!(false, validate_password("aabbcciol"));
    }
    #[test]
    fn test_generate_next_password() {
        assert_eq!(
            "abcdffaa",
            generate_next_password("abcdefgh".as_bytes().try_into().unwrap())
        );
    }
}
