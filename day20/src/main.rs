


// struct Solver {
//     value_cache: std::collections::HashMap<u32, u32>,
// }

// impl Solver {
//     pub fn new() -> Solver {
//         Solver{
//             value_cache: std::collections::HashMap::new(),
//             divisor_cache: std::collections::HashMap::new()
//         }
//     }

//     pub fn get_divisors(&mut self, number: u32) -> Vec<u32> {
//         (1..=number).filter(|v| number % v == 0).collect()
//     }

//     pub fn get_present_count_for_house(&mut self, house: u32) -> u32 {
//         divisor.iter().map(|e| value_cache.entry(e).or_insert(e*10)).sum()
//     }
// }



fn part_1(limit: u32) -> u32 {
    let limit = limit/10;
    //let mut s = Solver::new();
    let mut house_number = 1;
    loop {
        // if s.get_present_count_for_house(house_number) >= limit {
        //     break;
        // }
        // house_number += 1;
        // if house_number % 10000 == 0 {
        //     println!("After {} houses", house_number);
        // }
        let mut sum = 0;
        for i in 1..house_number {
            if i % house_number == 0 {
                sum += i;
            }
        }
        if sum >= limit {
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
        assert_eq!(vec![1,2], get_divisors(2));
        assert_eq!(vec![1,3], get_divisors(3));
        assert_eq!(vec![1,2,4], get_divisors(4));
        assert_eq!(vec![1,5], get_divisors(5));
        assert_eq!(vec![1,2,3,6], get_divisors(6));
        assert_eq!(vec![1,7], get_divisors(7));
        assert_eq!(vec![1,2,4,8], get_divisors(8));
        assert_eq!(vec![1,3,9], get_divisors(9));
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

