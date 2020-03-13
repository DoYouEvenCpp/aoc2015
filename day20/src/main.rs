
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
    //println!("First puzzle: {}", part_1(input));
    println!("Second puzzle: {}", part_2(input));
}
