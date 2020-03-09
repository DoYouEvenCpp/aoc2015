#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Velocity {
    velocity: u32,
    time: u32,
    rest: u32,
}

type DataType = std::collections::HashMap<String, Velocity>;
type Leaderboard = std::collections::HashMap<String, (u32, u32)>;

fn parse_input() -> DataType {
    let mut m = DataType::new();
    m.insert("Vixen".to_string(),   Velocity{velocity: 19, time: 7,    rest: 124});
    m.insert("Rudolph".to_string(), Velocity{velocity: 3,  time: 15,   rest: 28});
    m.insert("Donner".to_string(),  Velocity{velocity: 19, time: 9,    rest: 164});
    m.insert("Blitzen".to_string(), Velocity{velocity: 19, time: 9,    rest: 158});
    m.insert("Comet".to_string(),   Velocity{velocity: 13, time: 7,    rest: 82});
    m.insert("Cupid".to_string(),   Velocity{velocity: 25, time: 6,    rest: 145});
    m.insert("Dasher".to_string(),  Velocity{velocity: 14, time: 3,    rest: 38});
    m.insert("Dancer".to_string(),  Velocity{velocity: 3,  time: 16,   rest: 37});
    m.insert("Prancer".to_string(), Velocity{velocity: 25, time: 6,    rest: 143});
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
        let s = val.velocity * count * val.time;
        distances.insert(s);
    }
    *distances.iter().max().unwrap()
}

fn part_2(input: &mut DataType, t: u32) -> u32 {
    let mut current_leaderboard = Leaderboard::new();
    current_leaderboard.insert("Vixen".to_string(), (0, 0));
    current_leaderboard.insert("Rudolph".to_string(), (0, 0));
    current_leaderboard.insert("Donner".to_string(), (0, 0));
    current_leaderboard.insert("Blitzen".to_string(), (0, 0));
    current_leaderboard.insert("Comet".to_string(), (0, 0));
    current_leaderboard.insert("Cupid".to_string(), (0, 0));
    current_leaderboard.insert("Dasher".to_string(), (0, 0));
    current_leaderboard.insert("Dancer".to_string(), (0, 0));
    current_leaderboard.insert("Prancer".to_string(), (0, 0));

    for current_time in 0..=t {
        input.iter().for_each(|entry| {
            let the_rest = current_time % (entry.1.time + entry.1.rest);
            if the_rest < entry.1.time {
                //still bursting
                match current_leaderboard.get_mut(entry.0) {
                    Some(v) => v.0 += entry.1.velocity,
                    _ => panic!("xd"),
                }
            }
        });
        let current_max = current_leaderboard.iter().map(|(_, v)| v).max().unwrap().0;
        for val in &mut current_leaderboard.values_mut() {
            if val.0 == current_max {
                val.1 += 1;
            }
        }
    }
    current_leaderboard.iter().map(|(_, v)| v.1).max().unwrap()
}

fn main() {
    let mut data = parse_input();
    println!("First puzzle: {}", part_1(&data, 2503));
    println!("Second puzzle: {}", part_2(&mut data, 2503));
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
                velocity: 14,
                time: 10,
                rest: 127,
            },
        );
        data.insert(
            "Dancer".to_string(),
            Velocity {
                velocity: 16,
                time: 11,
                rest: 162,
            },
        );
        assert_eq!(1120, part_1(&data, 1000));
    }
}
