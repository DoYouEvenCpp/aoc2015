use md5;

fn compute_hash(input: &str, mask: u32) -> usize {
    for i in 0.. {
        let hash = md5::compute(format!("{}{}", input, i));
        let val = ((hash[0] as u32) << 16) + ((hash[1] as u32) << 8) + ((hash[2] as u32) << 0);
        if val & mask == 0 {
            return i;
        }
    }
    return 0;
}

fn part_1(input: &str) -> usize {
    let h = md5::compute(format!("{}{}", input, 117946));
    println!("res: {:x}", h);
    compute_hash(input, 0xFFFFF0)
}
fn part_2(input: &str) -> usize {
    compute_hash(input, 0xFFFFFF)
}

fn main() {
    let content = "ckczppom";
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day04 {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Running test for part 1");
        assert_eq!(609043, part_1("abcdef"));
        assert_eq!(1048970, part_1("pqrstuv"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
    }
}
