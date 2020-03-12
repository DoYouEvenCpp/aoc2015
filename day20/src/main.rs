
fn get_divisors(number: u32) -> Vec<u32> {
    (1..=number).filter(|v| number % v == 0).collect()
}

fn part_1(limit: u32) -> u32 {
    0
}

fn part_2(limit: u32) -> u32 {
    0
}

fn main() {
    let input = 33_100_000;
    println!("First puzzle: {}", part_1(input));
    println!("Second puzzle: {}", part_2(input));
}


#[cfg(test)]
mod day20 {
    use super::*;
    #[test]
    fn test_get_divisors() {
        assert_eq!(vec![1], get_divisors(1));
        assert_eq!(vec![1,2], get_divisors(2));
        assert_eq!(vec![1,3], get_divisors(3));
        assert_eq!(vec![1,2,4], get_divisors(4));
        assert_eq!(vec![1,5], get_divisors(5));
        assert_eq!(vec![1,2,3,6], get_divisors(6));
        assert_eq!(vec![1,7], get_divisors(7));
        assert_eq!(vec![1,2,4,8], get_divisors(8));
        assert_eq!(vec![1,3,9], get_divisors(9));
    }
}

