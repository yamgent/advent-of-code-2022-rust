const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
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

#[derive(Debug)]
struct Cpu {
    x: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 1 }
    }

    fn run(&mut self, instruction: &Instruction) {
        self.cycle = self.get_instruction_cycle_range(instruction).1 + 1;

        if let Instruction::Addx(value) = instruction {
            self.x += value;
        }
    }

    fn get_instruction_cycle_range(&self, instruction: &Instruction) -> (usize, usize) {
        match instruction {
            Instruction::Noop => (self.cycle, self.cycle),
            Instruction::Addx(..) => (self.cycle, self.cycle + 1),
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

        if cpu.get_instruction_cycle_range(&instruction).1 >= *cycles_left.iter().last().unwrap() {
            result += cycles_left.pop().unwrap() as i32 * cpu.x;
        }

        cpu.run(&instruction);
    }

    result.to_string()
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

fn p2(input: &str) -> String {
    let mut cpu = Cpu::new();
    let mut screen = std::iter::repeat(
        std::iter::repeat('.')
            .take(SCREEN_WIDTH)
            .collect::<Vec<_>>(),
    )
    .take(SCREEN_HEIGHT)
    .collect::<Vec<_>>();

    for instruction in input.trim().lines().map(Instruction::parse) {
        if cpu.cycle > SCREEN_SIZE {
            break;
        }

        let range = cpu.get_instruction_cycle_range(&instruction);

        (range.0..(range.1 + 1)).for_each(|cycle| {
            if cycle > SCREEN_SIZE {
                return;
            }

            let row = (cycle - 1) / SCREEN_WIDTH;
            let col = (cycle - 1) % SCREEN_WIDTH;

            if (col as i32 - cpu.x).abs() <= 1 {
                screen[row][col] = '#';
            }
        });

        cpu.run(&instruction);
    }

    screen
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
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
        assert_eq!(
            p2(SAMPLE_INPUT),
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .trim()
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(
            p2(ACTUAL_INPUT),
            r"
###..####.#..#.#....###...##..#..#..##..
#..#....#.#..#.#....#..#.#..#.#..#.#..#.
#..#...#..#..#.#....###..#..#.#..#.#..#.
###...#...#..#.#....#..#.####.#..#.####.
#....#....#..#.#....#..#.#..#.#..#.#..#.
#....####..##..####.###..#..#..##..#..#.
"
            .trim()
        );
    }
}
