
fn part_1(limit: u32) -> u32 {
    let limit = limit/10;
    let mut house_number = 1;
    let mut cache = std::collections::HashMap::new();
    loop {
        if (1..=house_number)
            .filter(|v| house_number % v == 0)
            .collect::<Vec<u32>>()
            .iter()
            .map(|e| cache.entry(*e).or_insert(*e).to_owned())
            .sum::<u32>()
            >= limit
        {
            break;
        }

        house_number += 1;
        if house_number % 10000 == 0 {
            println!("After {} houses", house_number);
        }
    }
    house_number
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
        assert_eq!(vec![1, 2], get_divisors(2));
        assert_eq!(vec![1, 3], get_divisors(3));
        assert_eq!(vec![1, 2, 4], get_divisors(4));
        assert_eq!(vec![1, 5], get_divisors(5));
        assert_eq!(vec![1, 2, 3, 6], get_divisors(6));
        assert_eq!(vec![1, 7], get_divisors(7));
        assert_eq!(vec![1, 2, 4, 8], get_divisors(8));
        assert_eq!(vec![1, 3, 9], get_divisors(9));
    }

    #[test]
    fn test_get_present_count_for_house() {
        assert_eq!(10, get_present_count_for_house(1));
        assert_eq!(30, get_present_count_for_house(2));
        assert_eq!(40, get_present_count_for_house(3));
        assert_eq!(70, get_present_count_for_house(4));
        assert_eq!(60, get_present_count_for_house(5));
        assert_eq!(120, get_present_count_for_house(6));
        assert_eq!(80, get_present_count_for_house(7));
        assert_eq!(150, get_present_count_for_house(8));
        assert_eq!(130, get_present_count_for_house(9));
    }
}
