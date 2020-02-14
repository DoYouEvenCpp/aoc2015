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
fn part_1(_input: &str) -> bool {
    true
}
fn part_2(_input: &str) -> bool {
    false
}
fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("{}", content);
    part_1(&content);
    part_2(&content);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("NO ELO 1");
        assert!(part_1("input_1"));
        assert_eq!(true, part_1("input_1"));
    }
    #[test]
    fn test_part_2() {
        println!("NO ELO 2");
        assert!(!part_2("input_2"));
        assert_eq!(false, part_2("input_2"));
    }
}
