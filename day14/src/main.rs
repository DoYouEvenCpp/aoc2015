
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Velocity {
    speed: u32,
    time: u32,
    rest: u32,
}

type DataType = std::collections::HashMap<String, Velocity>;

fn parse_input() -> DataType {
    let mut m = DataType::new();
    m.insert("Vixen".to_string(),   Velocity{speed: 19, time: 7,    rest: 124});
    m.insert("Rudolph".to_string(), Velocity{speed: 3,  time: 15,   rest: 28});
    m.insert("Donner".to_string(),  Velocity{speed: 19, time: 9,    rest: 164});
    m.insert("Blitzen".to_string(), Velocity{speed: 19, time: 9,    rest: 158});
    m.insert("Comet".to_string(),   Velocity{speed: 13, time: 7,    rest: 82});
    m.insert("Cupid".to_string(),   Velocity{speed: 25, time: 6,    rest: 145});
    m.insert("Dasher".to_string(),  Velocity{speed: 14, time: 3,    rest: 38});
    m.insert("Dancer".to_string(),  Velocity{speed: 3,  time: 16,   rest: 37});
    m.insert("Prancer".to_string(), Velocity{speed: 25, time: 6,    rest: 143});
    m
}

fn part_1(input: &DataType, time: u32) -> u32 {
    let mut distances = std::collections::HashSet::<u32>::new();
    for val in input.values() {
        let total_time = val.time + val.rest;
        let mut count = time / total_time;
        let modulo_rest = time % total_time;
        if modulo_rest >= val.time {
            count += 1;
        }
        let s = val.speed * count * val.time;
        distances.insert(s);
    }
    *distances.iter().max().unwrap()
}

fn part_2(input: &DataType) -> u32 {
    0
}

fn main() {
    let data = parse_input();
    println!("First puzzle: {}", part_1(&data, 2503));
    println!("Second puzzle: {}", part_1(&data, 2503));
}

#[cfg(test)]
mod day14 {
    use super::*;
    #[test]
    fn test_part_1() {
        let mut data = DataType::new();
        data.insert(
            "Comet".to_string(),
            Velocity {
                speed: 14,
                time: 10,
                rest: 127,
            },
        );
        data.insert(
            "Dancer".to_string(),
            Velocity {
                speed: 16,
                time: 11,
                rest: 162,
            },
        );
        assert_eq!(1120, part_1(&data, 1000));
    }
}
