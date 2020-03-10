fn part_1(input: &[u32], capacity: u32) -> u32 {
    0
}

fn part_2(input: &[u32], capacity: u32) -> u32 {
    0
}

fn main() {
    let input = [33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42];
    println!("part_1: {}", part_1(&input, 150));
    println!("part_2: {}", part_2(&input, 150));
}



#[cfg(test)]
mod day17 {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(4, part_1(&[5,5,10,15,20], 25));
    }
}