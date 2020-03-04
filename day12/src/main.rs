use std::fs;
use serde_json::{Result, Value};

fn part_1(i: &Value) -> i64 {
    let mut sum = 0;
    match i {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(|value| part_1(value)).sum::<i64>(),
        Value::Object(object) => sum += object.values().map(|value| part_1(value)).sum::<i64>(),
        _ => (),
    };
    sum
}

fn part_2(input: &Value) -> i32 {
   0
}

fn main() -> Result<()>{
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    let v: Value = serde_json::from_str(content)?;

    println!("First puzzle: {}", part_1(&v));
    println!("Second puzzle: {}", part_2(&v));
    Ok(())
}

#[cfg(test)]
mod day12 {
    use super::*;
}
