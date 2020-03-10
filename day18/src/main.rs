use std::fs;

type Map = Vec<Vec<char>>;

fn pase_input(input: &str) -> Map {
    let mut x: usize = 0;
    let mut y: usize = 0;
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
    x > 0 && x < 100 && y > 0 && y < 100
}

fn get_number_of_neighbours(state: char, x: usize, y: usize, map: &Map) -> u32 {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0)
    ].iter()
    .filter(|offset| does_exist(x as i32 + offset.0, y as i32 + offset.1))
    .filter(|offset| map[(x as i32 + offset.0) as usize][(y as i32 + offset.1) as usize] == '#')
    .count() as u32
}


fn part_1(input: &Map) -> u32 {
    let mut count: u32 = 0;
    let mut input = input;
    let mut tmp = input;
    for iteration in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                let number_of_neighbours = get_number_of_neighbours(input[x][y], x, y, &input);
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
                    _ => panic!("xD")
                }
            }
        }
        input = tmp;
    }
    for x in 0..100 {
        for y in 0..100 {
            if input[x][y] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn part_2(input: &Map) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let mut content = pase_input(content.trim());
    println!("First puzzle: {}", part_1(&mut content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day18 {
    use super::*;
}
