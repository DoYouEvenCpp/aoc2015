use std::fs;
use std::collections::HashMap;

type MapType = HashMap<String, Vec<String>>;

fn parse_input(line: &str, signals: &mut MapType) {
    let splitted: Vec<&str> = line.splitn(2, " -> ").collect();
    let key = splitted[1].to_string();
    let entry: Vec<String> = splitted[0].splitn(3, " ")
        .map(|s| s.to_owned())
        .collect();
    //println!("{} => \t\t{:?} => \t\t{:?}", line, splitted, entry);
    signals.insert(key, entry);
}

fn extract_value(key: &String, signals: &MapType) -> Vec<String> {
    if !signals.contains_key(key) {
        //most probably a value
        return vec![key.to_string()];
    }
    else {
        return signals.get(key).unwrap().to_vec();
    }
}

fn get_value(s: &Vec<String>, signals: &mut MapType) -> u32 {
    //println!("{:?}", s);
    if s.len() == 3 {
        let param1 = extract_value(&s[0], signals);
        let param2 = extract_value(&s[2], signals);
        match s[1].as_str() {
            "AND" => return get_value(&param1, signals) & get_value(&param2, signals),
            "OR" => return get_value(&param1, signals) | get_value(&param2, signals),
            "LSHIFT" => return get_value(&param1, signals) << get_value(&param2, signals),
            "RSHIFT" => return get_value(&param1, signals) >> get_value(&param2, signals),
            _ => 0
        }
    }
    else if s.len() == 2 {
        let param1 = extract_value(&s[1], signals);
        let val = get_value(&param1, signals);
        signals.insert(s[1].to_owned(), vec![val.to_string()]);
        return val;
    }
    else if s.len() == 1{
        if s[0].parse::<u32>().is_ok() {
            return s[0].parse::<u32>().unwrap();
        }
        else {
            let entry = extract_value(&s[0], signals);
            let val = get_value(&entry, signals);
            signals.insert(s[0].to_owned(), vec![val.to_string()]);
            println!("updated with {:?}", val);
            return val;
        }
    }
    else {
        panic!("dupa");
    }
}

fn part_1(input: &str) -> u32 {
    let mut map = MapType::new();
    for l in input.lines() {
        parse_input(l, &mut map);
    }
    // for (k, v) in &map{
    //     println!("{:?}<===>{:?}", k, v);
    // }

    let e = extract_value(&"a".to_string(), &map);
    get_value(&e, &mut map)
}

fn part_2(_input: &str) -> u32 {
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    println!("First puzzle: {}", part_1(&content));
    println!("Second puzzle: {}", part_2(&content));
}

#[cfg(test)]
mod day07 {
    use super::*;
    #[test]
    fn test_parse_input() {
        let mut map = MapType::new();
        parse_input("123 -> x", &mut map);
    }
}
