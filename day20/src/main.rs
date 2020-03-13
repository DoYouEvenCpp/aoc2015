fn part_1(presents_limit: usize) -> usize {
    let presents_limit = presents_limit / 10;
    let mut houses = vec![0; presents_limit];

    for elf in 0..presents_limit {
        (elf..presents_limit)
            .step_by(elf + 1)
            .for_each(|v| houses[v] += elf + 1);
    }

    houses.iter().position(|&p| p >= presents_limit).unwrap() + 1
}

fn part_2(presents_limit: usize) -> usize {
    let mut houses = vec![0; presents_limit];

    for elf in 0..presents_limit {
        (elf..presents_limit)
            .step_by(elf + 1)
            .take(50)
            .for_each(|v| houses[v] += (elf + 1) * 11);
    }

    houses.iter().position(|&p| p >= presents_limit).unwrap() + 1
}

fn main() {
    let input = 33_100_000;
    println!("First puzzle: {}", part_1(input));
    println!("Second puzzle: {}", part_2(input));
}
