#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

type DataType = std::collections::HashMap<String, Ingredient>;

fn get_input() -> DataType {
    let mut m = DataType::new();
    m.insert("Sprinkles".to_string(), Ingredient{capacity: 2, durability: 0, flavor: -2, texture: 0, calories: 3});
    m.insert("Butterscotch".to_string(), Ingredient{capacity: 0, durability: 5, flavor: -3, texture: 0, calories: 3});
    m.insert("Chocolate".to_string(), Ingredient{capacity: 0, durability: 0, flavor: 5, texture: -1, calories: 8});
    m.insert("Candy".to_string(), Ingredient{capacity: 0, durability: -1, flavor: 0, texture: 5, calories: 8});
    m
}

fn part_1(input: &DataType, time: u32) -> u32 {
    0
}

fn part_2(input: &DataType, t: u32) -> u32 {
    0
}

fn main() {
    let mut data = get_input();
    println!("First puzzle: {}", part_1(&data, 100));
    println!("Second puzzle: {}", part_2(&data, 100));
}

#[cfg(test)]
mod day15 {
    use super::*;
}