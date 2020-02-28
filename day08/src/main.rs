use std::fs;

fn get_total_characters_len(l: &str) -> usize {
    let l = &l[1..l.len() - 1];
    let mut count = 0;
    let mut current_pos = 0;
    while current_pos < l.len() {
        if l.chars().nth(current_pos).unwrap() == '\\' {
            if l.chars().nth(current_pos + 1).unwrap() == 'x' {
                current_pos += 3;
            } else {
                current_pos += 1;
            }
        }
        current_pos += 1;
        count += 1;
    }
    count
}

fn encode_characters(l: &str) -> usize {
    let predicate = |c: &char| c == &'"' || c == &'\\';
    l.chars().filter(predicate).count() + 2 + l.len()
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |sum, l| sum + (l.len() - get_total_characters_len(l)))
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .fold(0, |sum, l| sum + (encode_characters(l) - l.len()))
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
        assert_eq!(0, get_total_characters_len(r#""""#));
        assert_eq!(3, get_total_characters_len(r#""abc""#));
        assert_eq!(7, get_total_characters_len(r#""aaa\"aaa""#));
        assert_eq!(1, get_total_characters_len(r#""\x27""#));
    }

    #[test]
    fn test_encode_characters() {
        assert_eq!(6, encode_characters(r#""""#));
        assert_eq!(9, encode_characters(r#""abc""#));
        assert_eq!(16, encode_characters(r#""aaa\"aaa""#));
        assert_eq!(11, encode_characters(r#""\x27""#));
    }
}
