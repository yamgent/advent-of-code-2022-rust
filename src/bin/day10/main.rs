const ACTUAL_INPUT: &str = include_str!("./input.txt");

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        if line == "noop" {
            Instruction::Noop
        } else {
            let (opcode, value) = line.split_once(' ').unwrap();
            if opcode != "addx" {
                panic!("Unknown opcode {}", opcode);
            }
            Instruction::Addx(value.parse::<i32>().unwrap())
        }
    }
}

struct Cpu {
    x: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 1 }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
            }
            Instruction::Addx(value) => {
                self.cycle += 2;
                self.x += value;
            }
        }
    }
}

fn p1(input: &str) -> String {
    let mut cpu = Cpu::new();
    let mut cycles_left: Vec<usize> = vec![220, 180, 140, 100, 60, 20];
    let mut result = 0;

    for instruction in input.trim().lines().map(Instruction::parse) {
        if cycles_left.is_empty() {
            break;
        }

        let cycle_before_instruction_ends = match instruction {
            Instruction::Noop => cpu.cycle,
            Instruction::Addx(..) => cpu.cycle + 1,
        };

        if cycle_before_instruction_ends >= *cycles_left.iter().last().unwrap() {
            result += cycles_left.pop().unwrap() as i32 * cpu.x;
        }

        cpu.run(&instruction);
    }

    result.to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./sample.txt");

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "13140");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "11780");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
