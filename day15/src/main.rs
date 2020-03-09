use std::cmp;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

type DataType = Vec<Ingredient>;

fn get_input() -> DataType {
    let mut m = DataType::new();
    m.push(Ingredient {
        capacity: 2,
        durability: 0,
        flavor: -2,
        texture: 0,
        calories: 3,
    });
    m.push(Ingredient {
        capacity: 0,
        durability: 5,
        flavor: -3,
        texture: 0,
        calories: 3,
    });
    m.push(Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 5,
        texture: -1,
        calories: 8,
    });
    m.push(Ingredient {
        capacity: 0,
        durability: -1,
        flavor: 0,
        texture: 5,
        calories: 8,
    });
    m
}

fn calculate(
    input: &[Ingredient],
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> (i32, i32, i32, i32) {
    let capacity = cmp::max(
        0,
        first as i32 * input[0].capacity
            + second as i32 * input[1].capacity
            + third as i32 * input[2].capacity
            + fourth as i32 * input[3].capacity,
    );

    let durability = cmp::max(
        0,
        first as i32 * input[0].durability
            + second as i32 * input[1].durability
            + third as i32 * input[2].durability
            + fourth as i32 * input[3].durability,
    );

    let flavor = cmp::max(
        0,
        first as i32 * input[0].flavor
            + second as i32 * input[1].flavor
            + third as i32 * input[2].flavor
            + fourth as i32 * input[3].flavor,
    );

    let texture = cmp::max(
        0,
        first as i32 * input[0].texture
            + second as i32 * input[1].texture
            + third as i32 * input[2].texture
            + fourth as i32 * input[3].texture,
    );

    (capacity, durability, flavor, texture)
}

fn part_1(input: &[Ingredient], spoons: u32) -> u32 {
    let mut vals = std::collections::HashSet::<u32>::new();
    for first in 0..spoons {
        for second in 0..spoons - first {
            for third in 0..spoons - first - second {
                let fourth = spoons - first - second - third;
                let v = calculate(input, first, second, third, fourth);
                let sum = v.0 * v.1 * v.2 * v.3;
                vals.insert(sum as u32);
            }
        }
    }
    *vals.iter().max().unwrap()
}

fn part_2(input: &[Ingredient], spoons: u32) -> u32 {
    let mut vals = std::collections::HashSet::<u32>::new();
    for first in 0..spoons {
        for second in 0..spoons - first {
            for third in 0..spoons - first - second {
                let fourth = spoons - first - second - third;

                let calories = cmp::max(
                    0,
                    first as i32 * input[0].calories
                        + second as i32 * input[1].calories
                        + third as i32 * input[2].calories
                        + fourth as i32 * input[3].calories,
                );

                if calories == 500 {
                    let v = calculate(input, first, second, third, fourth);
                    let sum = v.0 * v.1 * v.2 * v.3;
                    vals.insert(sum as u32);
                }
            }
        }
    }
    *vals.iter().max().unwrap()
}

fn main() {
    let data = get_input();
    println!("First puzzle: {}", part_1(&data, 100));
    println!("Second puzzle: {}", part_2(&data, 100));
}
