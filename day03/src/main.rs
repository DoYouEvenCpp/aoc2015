use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

type MapType = HashMap<Position, u32>;

fn handle_move(m: char, map: &mut MapType, current_pos: Position) -> Position {
    let mut pos = current_pos;
    match m {
        '^' => pos.x -= 1,
        'v' => pos.x += 1,
        '>' => pos.y += 1,
        '<' => pos.y -= 1,
        _ => (),
    }
    if let Some(val) = map.get_mut(&pos) {
        *val += 1;
    } else {
        map.insert(pos, 1);
    }
    pos
}

fn part_1(input: &str) -> u32 {
    let mut current_position = Position { x: 0, y: 0 };
    let mut map: MapType = HashMap::new();
    map.insert(current_position, 1);

    for c in input.chars() {
        current_position = handle_move(c, &mut map, current_position);
    }

    let mut counter: u32 = 0;
    for (_, value) in map.into_iter() {
        if value >= 1 {
            counter += 1;
        }
    }
    counter
}
fn part_2(input: &str) -> u32 {
    let mut santa_position = Position { x: 0, y: 0 };
    let mut robo_santa_position = Position { x: 0, y: 0 };
    let mut map: MapType = HashMap::new();
    let mut counter: u32 = 0;
    map.insert(santa_position, 2);

    for c in input.chars() {
        if counter % 2 == 0 {
            santa_position = handle_move(c, &mut map, santa_position);
        } else {
            robo_santa_position = handle_move(c, &mut map, robo_santa_position);
        }
        counter += 1;
    }

    counter = 0;
    for (_, value) in map.iter() {
        if value >= &1 {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day01 {
    use super::*;
    #[test]
    fn test_part_1() {
        println!("Running test for part 1");
        assert_eq!(2, part_1("^v"));
        assert_eq!(4, part_1("^>v<"));
        assert_eq!(2, part_1("^v^v^v^v^v"));
    }
    #[test]
    fn test_part_2() {
        println!("Running test for part 2");
        assert_eq!(3, part_2("^v"));
        assert_eq!(3, part_2("^>v<"));
        assert_eq!(11, part_2("^v^v^v^v^v"));
    }
}
