use std::fs;

type Map = Vec<Vec<char>>;

fn pase_input(input: &str) -> Map {
    let mut data = Map::new();
    for line in input.lines() {
        data.push(vec![]);
        for c in line.chars() {
            data.last_mut().unwrap().push(c);
        }
    }
    data.truncate(input.lines().count());
    data
}

fn does_exist(x: i32, y: i32) -> bool {
    x >= 0 && x < 100 && y >= 0 && y < 100
}

fn get_number_of_neighbours(x: usize, y: usize, map: &Map) -> u32 {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter(|offset| does_exist(x as i32 + offset.0, y as i32 + offset.1))
    .filter(|offset| map[(x as i32 + offset.0) as usize][(y as i32 + offset.1) as usize] == '#')
    .count() as u32
}

fn get_number_of_lit_ligts(map: &Map) -> u32 {
    map.into_iter()
        .flatten()
        .filter(|ch| **ch == '#')
        .fold(0, |sum, _| sum + 1)
}

fn part_1(mut input: Map) -> u32 {
    let mut tmp = input.clone();
    for _ in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                let number_of_neighbours = get_number_of_neighbours(x, y, &input);
                match input[x][y] {
                    '#' => {
                        if number_of_neighbours < 2 || number_of_neighbours > 3 {
                            tmp[x][y] = '.';
                        }
                    }
                    '.' => {
                        if number_of_neighbours == 3 {
                            tmp[x][y] = '#';
                        }
                    }
                    _ => panic!("xD"),
                }
            }
        }
        input = tmp.clone();
    }
    get_number_of_lit_ligts(&input)
}

fn part_2(mut input: Map) -> u32 {
    let mut tmp = input.clone();
    for _ in 0..100 {
        input[0][0] = '#';
        input[99][0] = '#';
        input[0][99] = '#';
        input[99][99] = '#';
        for x in 0..100 {
            for y in 0..100 {
                let number_of_neighbours = get_number_of_neighbours(x, y, &input);
                match input[x][y] {
                    '#' => {
                        if number_of_neighbours < 2 || number_of_neighbours > 3 {
                            tmp[x][y] = '.';
                        }
                    }
                    '.' => {
                        if number_of_neighbours == 3 {
                            tmp[x][y] = '#';
                        }
                    }
                    _ => panic!("xD"),
                }
            }
        }
        tmp[0][0] = '#';
        tmp[99][0] = '#';
        tmp[0][99] = '#';
        tmp[99][99] = '#';
        input = tmp.clone();
    }
    get_number_of_lit_ligts(&input)
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = pase_input(content.trim());
    println!("First puzzle: {}", part_1(content.clone()));
    println!("Second puzzle: {}", part_2(content));
}
