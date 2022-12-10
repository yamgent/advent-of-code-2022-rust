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
    let mut cpu = Cpu::new();
    let mut screen = std::iter::repeat('.').take(240).collect::<Vec<_>>();

    for instruction in input.trim().lines().map(Instruction::parse) {
        if cpu.cycle > screen.len() {
            break;
        }

        let cycle_before_instruction_ends = match instruction {
            Instruction::Noop => cpu.cycle,
            Instruction::Addx(..) => cpu.cycle + 1,
        };

        (cpu.cycle..(cycle_before_instruction_ends + 1)).for_each(|cycle| {
            if cycle > screen.len() {
                return;
            }

            let pos = ((cycle - 1) % 40) as i32;

            if (pos - cpu.x).abs() <= 1 {
                screen[cycle - 1] = '#';
            }
        });

        cpu.run(&instruction);
    }

    screen
        .chunks(40)
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
