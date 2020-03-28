use std::fs;

struct Computer {
    memory: Vec<String>,
    reg_a: usize,
    reg_b: usize,
    pc: i32,
}


impl Computer {
    fn new(memory: &str) -> Self {
        Computer {
            memory: memory.lines().map(String::from).collect(),
            reg_a: 0,
            reg_b: 0,
            pc: 0,
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.memory.len() as i32 {
                break;
            }

            let cmd: Vec<&str> = self.memory[self.pc as usize].split(' ').collect();
            match cmd[0] {
                "hlf" => {
                    let register = &cmd[1].trim_end_matches(',').to_owned();
                    self.half(register);
                }
                "tpl" => {
                    let register = &cmd[1].trim_end_matches(',').to_owned();
                    self.tripple(register);
                }
                "inc" => {
                    let register = &cmd[1].trim_end_matches(',').to_owned();
                    self.increment(register);
                }
                "jmp" => {
                    let offset = cmd[1].parse::<i32>().unwrap(); 
                    self.jump(offset);
                }
                "jie" => {
                    let register = &cmd[1].trim_end_matches(',').to_owned();
                    let offset = cmd[2].parse::<i32>().unwrap();
                    self.jump_if_even(register, offset);
                }
                "jio" => {
                    let register = &cmd[1].trim_end_matches(',').to_owned();
                    let offset = cmd[2].parse::<i32>().unwrap();
                    self.jump_if_one(register, offset);
                }
                _ => panic!()
            }

        }
    }
    fn half(&mut self, r: &str) {
        match r {
            "a" => self.reg_a /= 2,
            "b" => self.reg_b /= 2,
            _ => panic!()
        }
        self.pc += 1;
    }

    fn tripple(&mut self, r: &str) {
        match r {
            "a" => self.reg_a *= 3,
            "b" => self.reg_b *= 3,
            _ => panic!()
        }
        self.pc += 1
    }

    fn increment(&mut self, r: &str) {
        match r {
            "a" => self.reg_a += 1,
            "b" => self.reg_b += 1,
            _ => panic!()
        }
        self.pc += 1
    }

    fn jump(&mut self, step: i32) {
        self.pc += step
    }

    fn jump_if_even(&mut self, r: &str, step: i32) {
        match r {
            "a" => {
                if self.reg_a % 2 == 0 {
                    self.pc += step
                }
                else {
                    self.pc += 1;
                }
            }
            "b" => {
                if self.reg_b % 2 == 0 {
                    self.pc += step
                }
                else {
                    self.pc += 1;
                }
            }
            _ => panic!()
        }
    }

    fn jump_if_one(&mut self, r: &str, step: i32) {
        match r {
            "a" => {
                if self.reg_a == 1 {
                    self.pc += step
                }
                else {
                    self.pc += 1;
                }
            }
            "b" => {
                if self.reg_b == 1 {
                    self.pc += step
                }
                else {
                    self.pc += 1;
                }
            }
            _ => panic!()
        }
    }
    
}

fn part_1(input: &str) -> usize {
    let mut pc = Computer::new(input);
    pc.run();
    pc.reg_b
}

fn part_2(input: &str) -> usize {
    let mut pc = Computer::new(input);
    pc.reg_a = 1;
    pc.run();
    pc.reg_b
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}


#[cfg(test)]
mod day23 {
    use super::*;

    #[test]
    fn test_pc() {
        let code = "inc a\n\
                       jio a, +2\n\
                       tpl a\n\
                       inc a";
        let mut pc = Computer::new(code);
        pc.run();

        assert_eq!(2, pc.reg_a);
    }
}