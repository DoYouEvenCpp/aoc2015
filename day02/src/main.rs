use std::fs;

fn get_dimensions(line: &str) -> Vec<u32> {
    let mut dims: Vec<u32> = line
        .split('x')
        .map(|dimensions| dimensions.parse().unwrap())
        .collect();
    dims.sort();
    dims
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let dim = get_dimensions(l);
            let (a, b, c) = (dim[0], dim[1], dim[2]);
            2 * a * b + 2 * a * c + 2 * b * c + a * b
        })
        .sum()
}
fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let dim = get_dimensions(l);
            let (a, b, c) = (dim[0], dim[1], dim[2]);
            2 * a + 2 * b + a * b * c
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day02 {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Running test for part 1");
        assert_eq!(58, part_1("2x3x4"));
        assert_eq!(43, part_1("1x1x10"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
        assert_eq!(34, part_2("2x3x4"));
        assert_eq!(14, part_2("1x1x10"));
    }
}
