pub fn solve() {
    println!("--- Day 2: Dive! ---");
    let instructions = input();
    let mut sub = Submarine::new();
    for instruction in instructions.clone() {
        sub.execute(instruction);
    }

    println!(
        "\tsilver - horizontal position {}",
        sub.horizontal_position()
    );

    let mut sub = Submarine::new();
    for instruction in instructions {
        sub.execute_with_aim(instruction);
    }
    println!(
        "\tgold   - horizontal position {}",
        sub.horizontal_position()
    );
}

#[derive(Clone, Copy)]
enum Instruction {
    Down(usize),
    Up(usize),
    Forward(usize),
}

#[derive(Clone, Copy, Debug)]
struct Submarine {
    x: isize,
    z: isize,
    aim: isize,
}

impl Submarine {
    pub fn new() -> Self {
        Self { x: 0, z: 0, aim: 0 }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Down(value) => self.z -= value as isize,
            Instruction::Up(value) => self.z += value as isize,
            Instruction::Forward(value) => self.x += value as isize,
        }
    }

    pub fn execute_with_aim(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Down(value) => self.aim += value as isize,
            Instruction::Up(value) => self.aim -= value as isize,
            Instruction::Forward(value) => {
                self.x += value as isize;
                self.z -= value as isize * self.aim;
            }
        }
    }

    pub fn horizontal_position(&self) -> usize {
        (self.x * self.z).abs() as usize
    }
}

fn input() -> Vec<Instruction> {
    let input_string = crate::util::file2str("inputs/day02_dive.txt");
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input_string.lines() {
        if let Some(instruction) = line2instruction(line) {
            instructions.push(instruction);
        }
    }
    instructions
}

fn line2instruction(line: &str) -> Option<Instruction> {
    let mut splitted = line.split_whitespace();
    let instruction_type_str = splitted.next()?;
    let instruction_size_str = splitted.next()?;
    let instruction_size = instruction_size_str.parse::<usize>().unwrap();
    match instruction_type_str {
        "forward" => return Some(Instruction::Forward(instruction_size)),
        "down" => return Some(Instruction::Down(instruction_size)),
        "up" => return Some(Instruction::Up(instruction_size)),
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn instructions() -> Vec<Instruction> {
        vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ]
    }

    #[test]
    fn initial_is_zero() {
        let sub = Submarine::new();
        assert_eq!(sub.horizontal_position(), 0);
    }

    #[test]
    fn silver_example_test() {
        let mut sub = Submarine::new();
        for instruction in instructions() {
            sub.execute(instruction);
        }
        assert_eq!(sub.horizontal_position(), 150);
    }

    #[test]
    fn gold_example_test() {
        let mut sub = Submarine::new();
        for instruction in instructions() {
            sub.execute_with_aim(instruction);
        }
        assert_eq!(sub.horizontal_position(), 900);
    }
}
