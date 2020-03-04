fn single_iteration(input: &Vec<u32>) -> Vec<u32> {
    let mut numbers: Vec<(u32, u32)> = vec![];
    for ch in input {
        if numbers.is_empty() {
            numbers.push((*ch, 1));
        } else if numbers.last().unwrap().0 == *ch {
            numbers.last_mut().unwrap().1 += 1;
        } else {
            numbers.push((*ch, 1));
        }
    }
    let mut res: Vec<u32> = vec![];
    for (ch, count) in &numbers {
        res.push(*count);
        res.push(*ch);
    }
    res
}

fn part_1(input: &str) -> usize {
    let digits = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut res = digits;
    for _ in 0..40 {
        res = single_iteration(&res);
    }
    res.len()
}

fn part_2(input: &str) -> usize {
    let digits = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut res = digits;
    for _ in 0..50 {
        res = single_iteration(&res);
    }
    res.len()
}

fn main() {
    let content = "1321131112";
    println!("First puzzle: {:?}", part_1(&content));
    println!("Second puzzle: {:?}", part_2(&content));
}

#[cfg(test)]
mod day10 {
    use super::*;
    #[test]
    fn test_single_iteration() {
        assert_eq!(
            [1, 1],
            single_iteration(
                &"1".chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            )
            .as_slice()
        );
        assert_eq!(
            [2, 1],
            single_iteration(
                &"11"
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            )
            .as_slice()
        );
        assert_eq!(
            [1, 2, 1, 1],
            single_iteration(
                &"21"
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            )
            .as_slice()
        );
        assert_eq!(
            [1, 1, 1, 2, 2, 1],
            single_iteration(
                &"1211"
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            )
            .as_slice()
        );
        assert_eq!(
            [3, 1, 2, 2, 1, 1],
            single_iteration(
                &"111221"
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            )
            .as_slice()
        );
    }
}
