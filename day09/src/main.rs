use std::fs;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Edge {
    vertex: String,
    distance: usize
}

type MapType = std::collections::HashMap<String, Vec<Edge>>;

fn handle_entry(line: &str, map: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " = ").collect();
    let distance: usize = splitted[1].to_string().parse().unwrap();
    let towns: Vec<String> = splitted[0].splitn(2, " to ")
        .map(|s| s.to_owned())
        .collect();
    //println!("{} to {} in {}", towns[0], towns[1], distance);
    let e = map.entry(towns[0].to_owned()).or_default();
    (*e).push(Edge{vertex: towns[1].to_owned(), distance});

    let e = map.entry(towns[1].to_owned()).or_default();
    (*e).push(Edge{vertex: towns[0].to_owned(), distance});
}

fn parse_input(input: &str) -> MapType {
    let mut map = MapType::new();
    for line in input.lines() {
        handle_entry(line, &mut map);
    }
    map
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let map = parse_input(content);

    for (town, edges) in &map {
        print!("{}: ", town);
        for e in edges {
            print!("{}({}), ", e.vertex, e.distance)
        }
        println!();
    }

    println!("\nPermutations\n");
    let mut distances = std::collections::HashSet::new();
    for (start, vertexes) in map {
        for perm in vertexes.iter().permutations(vertexes.len()).unique() {
            //println!("{:?}\n\n", perm[0]);
            //  for (edge, vertexes) in perm {
                    let sum = perm.iter().fold(0, |acc, edge| acc + edge.distance);
                    distances.insert(sum);
            //      print!("{} -> ", edge);
            //      for edge in vertexes {
            //          print!("{}({}), ", edge.vertex, edge.distance);
            //      }
            //      println!();
            //  }
            //println!("{:?}", perm);
        }
    }
    // for perm in map.iter().permutations(map.len()).unique() {
    //     //println!("{:?}\n\n", perm[0]);
    //      for (edge, vertexes) in perm {
    //          let sum = vertexes.iter().fold(0, |acc, edge| acc + edge.distance);
    //          distances.insert(sum);
    //          print!("{} -> ", edge);
    //          for edge in vertexes {
    //              print!("{}({}), ", edge.vertex, edge.distance);
    //          }
    //          println!();
    //      }
    // }


    println!("First puzzle: {:?}", distances);
    println!("Second puzzle: {}", part_2(&content));

    //333 NOK!
}

#[cfg(test)]
mod day09 {
    use super::*;
}
