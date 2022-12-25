const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/05/input.txt");

struct TestCase {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

struct Instruction {
    amount: usize,
    source: usize,
    destination: usize,
}

impl TestCase {
    fn parse_input(input: &str) -> Self {
        let mut input = input.split("\n\n");

        let start_state = input.next().unwrap();
        let instructions = input.next().unwrap();

        let total_stacks = start_state
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks = std::iter::repeat(vec![])
            .take(total_stacks)
            .collect::<Vec<_>>();

        start_state.lines().rev().skip(1).for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .zip(stacks.iter_mut())
                .filter(|(character, _)| *character != ' ')
                .for_each(|(character, stack)| stack.push(character));
        });

        Self {
            stacks,
            instructions: instructions
                .trim()
                .lines()
                .map(|line| {
                    let mut parts = line.split_whitespace().skip(1).step_by(2);

                    Instruction {
                        amount: parts.next().unwrap().parse().unwrap(),
                        source: parts.next().unwrap().parse::<usize>().unwrap() - 1,
                        destination: parts.next().unwrap().parse::<usize>().unwrap() - 1,
                    }
                })
                .collect(),
        }
    }
}

fn transform_to_result(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn p1(input: &str) -> String {
    let TestCase {
        mut stacks,
        instructions,
    } = TestCase::parse_input(input);

    instructions.into_iter().for_each(
        |Instruction {
             amount,
             source,
             destination,
         }| {
            (0..amount).for_each(|_| {
                let value = stacks[source].pop().unwrap();
                stacks[destination].push(value);
            });
        },
    );

    transform_to_result(stacks)
}

fn p2(input: &str) -> String {
    let TestCase {
        mut stacks,
        instructions,
    } = TestCase::parse_input(input);

    instructions.into_iter().for_each(
        |Instruction {
             amount,
             source,
             destination,
         }| {
            let remaining_amount = stacks[source].len() - amount;
            let mut tail = stacks[source].split_off(remaining_amount);
            stacks[destination].append(&mut tail);
        },
    );

    transform_to_result(stacks)
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
        assert_eq!(p1(SAMPLE_INPUT), "CMZ");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "GFTNRBZPF");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "MCD");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "VRQWPDSGP");
    }
}
