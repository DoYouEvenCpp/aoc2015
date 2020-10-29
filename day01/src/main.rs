/*
 * rustup update // update srodowiska
 * cargo new day01
 * echo "file input" > ./day01/input
 * cd day01
 *
 * // uruchamia main()
 * cargo run
 * cargo run --release
 *
 * // uruchamia testy
 * cargo test
 * cargo test part_1
 * cargo test -- --nocapture
 * cargo test -- --test-threads=1
 *
 * // formatuje kod
 * cargo fmt
 */
use std::fs;
fn part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, ch|{
        match ch {
            ')' => acc - 1,
            '(' => acc + 1,
            _ => acc,
        }
    })
}
fn part_2(_input: &str) -> usize {
    let mut level: i32 = 0;
    let mut index: usize = 0;
    for c in _input.chars() {
        match c {
            ')' => level += -1,
            '(' => level += 1,
            _ => (),
        }
        index += 1;
        if level == -1 {
            println!("Second puzzle {}", index);
            return index;
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    part_1(&content);
    part_2(&content);
}

#[cfg(test)]
mod day01 {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Running test for part 1");
        assert_eq!(-3, part_1(")))"));
        assert_eq!(0, part_1(")))((("));
        assert_eq!(5, part_1("((((("));
        assert_eq!(0, part_1(""));
        assert_eq!(0, part_1("dfgsdgsdf"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
        assert_eq!(1, part_2(")"));
        assert_eq!(5, part_2("()())"));
        assert_eq!(0, part_2("fdgsdfgsfd"));
        assert_eq!(0, part_2(""));
    }
}
