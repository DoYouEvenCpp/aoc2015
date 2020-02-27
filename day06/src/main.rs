use std::fs;

type Position = (usize, usize, usize, usize);

fn extract_positions(line: &str) -> Position {
    let v: Vec<usize> = line
        .replace(" through ", ",")
        .splitn(4, ',')
        .map(|s| s.to_owned().parse().unwrap())
        .collect();
    (v[0], v[1], v[2], v[3])
}

fn part_1(input: &str) -> u32 {
    let turn_off = "turn off ";
    let turn_on = "turn on ";
    let toggle = "toggle ";

    let mut map = vec![vec![0; 1000]; 1000];

    for l in input.lines() {
        if l.contains(turn_on) {
            let p = extract_positions(l.split_at(turn_on.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    map[i][j] = 1;
                }
            }
        } else if l.contains(turn_off) {
            let p = extract_positions(l.split_at(turn_off.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    map[i][j] = 0;
                }
            }
        } else if l.contains(toggle) {
            let p = extract_positions(l.split_at(toggle.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    if map[i][j] == 1 {
                        map[i][j] = 0;
                    } else {
                        map[i][j] = 1;
                    }
                }
            }
        }
    }

    map.iter().fold(0, |acc: u32, line: &Vec<u32>| {
        acc + line.iter().sum::<u32>()
    })
}

fn part_2(input: &str) -> u32 {
    let turn_off = "turn off ";
    let turn_on = "turn on ";
    let toggle = "toggle ";

    let mut map = vec![vec![0; 1000]; 1000];

    for l in input.lines() {
        if l.contains(turn_on) {
            let p = extract_positions(l.split_at(turn_on.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    map[i][j] += 1;
                }
            }
        } else if l.contains(turn_off) {
            let p = extract_positions(l.split_at(turn_off.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    if map[i][j] > 0 {
                        map[i][j] -= 1;
                    }
                }
            }
        } else if l.contains(toggle) {
            let p = extract_positions(l.split_at(toggle.len()).1);
            for i in p.0..=p.2 {
                for j in p.1..=p.3 {
                    map[i][j] += 2;
                }
            }
        }
    }

    map.iter().fold(0, |acc: u32, line: &Vec<u32>| {
        acc + line.iter().sum::<u32>()
    })
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day05 {
    use super::*;
    #[test]
    fn test_extract_positions() {
        assert_eq!(
            (660, 55, 986, 197),
            extract_positions("660,55 through 986,197")
        );
    }
}
