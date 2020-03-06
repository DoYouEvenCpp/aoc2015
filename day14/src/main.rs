use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Velocity {
    speed: u32,
    time: u32,
    rest: u32,
}

type DataType = std::collections::HashMap<String, Velocity>;
type Leaderboard = std::collections::HashMap<String, u32>;

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

fn part_2(input: &mut DataType, t: u32) -> u32 {
    let mut current_leaderboard = Leaderboard::new();
    current_leaderboard.insert("Vixen".to_string(), 0);
    current_leaderboard.insert("Rudolph".to_string(), 0);
    current_leaderboard.insert("Donner".to_string(), 0);
    current_leaderboard.insert("Blitzen".to_string(), 0);
    current_leaderboard.insert("Comet".to_string(), 0);
    current_leaderboard.insert("Cupid".to_string(), 0);
    current_leaderboard.insert("Dasher".to_string(), 0);
    current_leaderboard.insert("Dancer".to_string(), 0);
    current_leaderboard.insert("Prancer".to_string(), 0);

    for current_time in 1..=t {
        input.iter().for_each(|entry| {
            let the_rest = current_time % (entry.1.time + entry.1.rest);
            if the_rest <= entry.1.time {
                //still bursting
                match current_leaderboard.get_mut(entry.0) {
                    Some(v) => *v += entry.1.speed,
                    _ => panic!("xd")
                }
            }
        });
        let current_max = *current_leaderboard.iter().map(|(_, v)| v).max().unwrap();
        //println!("{} {:?} {}",current_time, current_leaderboard, current_max);
        //current_leaderboard.iter_mut().filter(|p| p.1 == &mut current_max).map(|p| *p.1 += 1);
        for (_, val) in &mut current_leaderboard {
            if *val == current_max {
                *val += 1;
            }
        }
        println!("{} {:?} {}",current_time, current_leaderboard, current_max);
    }
    //817 - too low
    //3277 - too high
    0
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
