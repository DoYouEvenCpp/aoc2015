use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Relationship {
    name: String,
    val: i32,
}

type MapType = HashMap<String, Vec<Relationship>>;

fn parse_input(input: &str) -> MapType {
    let mut m = MapType::new();
    for line in input.lines() {
        let splitted: Vec<&str> = line.splitn(2, "next to ").collect();
        let other_person = splitted[1];
        let other_person = other_person.replace('.', "");
        let name_value: Vec<&str> = splitted[0].splitn(2, " happiness").collect();
        let name = name_value[0].splitn(2, " would").next().unwrap().trim();
        let value = name_value[0]
            .splitn(2, " would ")
            .nth(1)
            .map(|s| match s.split_at("gain".len()).0 {
                "gain" => s.split_at("gain ".len()).1.parse::<i32>().unwrap(),
                "lose" => -s.split_at("gain ".len()).1.parse::<i32>().unwrap(),
                _ => panic!("xD"),
            })
            .unwrap();
        let e = m.entry(name.to_string()).or_default();
        (*e).push(Relationship {
            name: other_person,
            val: value,
        });
    }
    m
}

fn get_happiness_value(n: &String, relationships: &Vec<Relationship>) -> i32 {
    relationships.iter().find(|r| &r.name == n).unwrap().val
}

fn part_1(input: &MapType) -> u32 {
    let vals = std::collections::HashSet::<i32>::new();
    for (name, values) in input {
        for p in values.iter().permutations(values.len()).unique() {

            //TODO: REFACTOR
            //ugly as fuck :|
            let mut tmp  = vec![name.to_string()];
            let mut dd: Vec<String> = p.iter().map(|r| r.name.to_owned()).collect();
            let mut ee  = vec![name.to_string()];
            tmp.append(&mut dd);
            tmp.append(&mut ee);
            //REFACTOR UNTIL HERE

            //CONTINUE BELOW
            let sum = tmp.windows(2).enumerate().fold(0, |acc, p| {
                acc + get
            })
            println!("{} - {:?}", name, tmp);
        }
    }

    // for (start, vertexes) in &map {
    //     for perm in vertexes.iter().permutations(vertexes.len()).unique() {
    //         current_town = start.to_owned();
    //         let mut sum = 0;
    //         for edge in perm {
    //             sum += map
    //                 .get(&current_town)
    //                 .unwrap()
    //                 .iter()
    //                 .filter(|e| e.vertex == edge.vertex)
    //                 .fold(0, |acc, e| acc + e.distance);
    //             current_town = edge.vertex.to_owned();
    //         }
    //         distances.insert(sum);
    //     }
    // }
    0
}

fn part_2(_input: &MapType) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let input = parse_input(
        "Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7 happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.",
    );
    println!("First puzzle: {}", part_1(&input));
    println!("Second puzzle: {}", part_2(&input));
}

#[cfg(test)]
mod day13 {
    use super::*;
    #[test]
    fn test_parse_input() {
        let m = parse_input(
            "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.",
        );
        assert_eq!(4, m.len());
        assert_eq!(true, m.contains_key(&"Alice".to_string()));
        assert_eq!(true, m.contains_key(&"Bob".to_string()));
        assert_eq!(true, m.contains_key(&"Carol".to_string()));
        assert_eq!(true, m.contains_key(&"David".to_string()));
        assert_eq!(
            vec![
                Relationship {
                    name: "Bob".to_string(),
                    val: 54
                },
                Relationship {
                    name: "Carol".to_string(),
                    val: -79
                },
                Relationship {
                    name: "David".to_string(),
                    val: -2
                }
            ],
            m.get("Alice").unwrap().as_slice()
        );
    }

    #[test]
    fn test_get_happiness_value() {
        let m = parse_input(
            "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.",
        );
        let relationships = m.get("Alice").unwrap();
        assert_eq!(54, get_happiness_value(&"Bob".to_owned(), &relationships));
        assert_eq!(
            -79,
            get_happiness_value(&"Carol".to_owned(), &relationships)
        );
        assert_eq!(-2, get_happiness_value(&"David".to_owned(), &relationships));
    }
}
