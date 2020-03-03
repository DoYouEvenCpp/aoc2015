use itertools::Itertools;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Edge {
    vertex: String,
    distance: usize,
}

type MapType = std::collections::HashMap<String, Vec<Edge>>;

fn handle_entry(line: &str, map: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " = ").collect();
    let distance: usize = splitted[1].to_string().parse().unwrap();
    let towns: Vec<String> = splitted[0]
        .splitn(2, " to ")
        .map(|s| s.to_owned())
        .collect();
    let e = map.entry(towns[0].to_owned()).or_default();
    (*e).push(Edge {
        vertex: towns[1].to_owned(),
        distance,
    });

    let e = map.entry(towns[1].to_owned()).or_default();
    (*e).push(Edge {
        vertex: towns[0].to_owned(),
        distance,
    });
}

fn parse_input(input: &str) -> MapType {
    let mut map = MapType::new();
    for line in input.lines() {
        handle_entry(line, &mut map);
    }
    map
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let map = parse_input(content);

    //for sure that is UGLY
    let mut distances = std::collections::HashSet::new();
    let mut current_town: String;
    for (start, vertexes) in &map {
        for perm in vertexes.iter().permutations(vertexes.len()).unique() {
            current_town = start.to_owned();
            let mut sum = 0;
            for edge in perm {
                sum += map
                    .get(&current_town)
                    .unwrap()
                    .iter()
                    .filter(|e| e.vertex == edge.vertex)
                    .fold(0, |acc, e| acc + e.distance);
                current_town = edge.vertex.to_owned();
            }
            distances.insert(sum);
        }
    }

    println!("First puzzle: {}", distances.iter().min().unwrap());
    println!("Second puzzle: {}", distances.iter().max().unwrap());
}
