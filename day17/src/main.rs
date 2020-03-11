type DataType = std::collections::HashMap<u32, u32>;

fn knapsack(
    data: &[u32],
    limit: u32,
    current_weight: u32,
    count: &mut DataType,
    containers_used: u32,
) {
    for (idx, _) in data.iter().enumerate() {
        let new_weight = current_weight + data[idx];
        if new_weight == limit {
            let combination = count.entry(containers_used).or_insert(0);
            *combination += 1;
            continue;
        } else if new_weight < limit {
            knapsack(
                &data[idx + 1..],
                limit,
                new_weight,
                count,
                containers_used + 1,
            );
        }
    }
}

fn part_1(input: &[u32], capacity: u32) -> u32 {
    let mut combinations = DataType::new();
    knapsack(input, capacity, 0, &mut combinations, 0);
    combinations.iter().fold(0, |acc, p| acc + p.1)
}

fn part_2(input: &[u32], capacity: u32) -> u32 {
    let mut combinations = DataType::new();
    knapsack(input, capacity, 0, &mut combinations, 0);
    *combinations.iter().min().unwrap().1
}

fn main() {
    let input = [
        33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
    ];
    println!("part_1: {}", part_1(&input, 150));
    println!("part_2: {}", part_2(&input, 150));
}

#[cfg(test)]
mod day17 {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(4, part_1(&[5, 5, 10, 15, 20], 25));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, part_2(&[5, 5, 10, 15, 20], 25));
    }
}
