use itertools::Itertools;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

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
        let (total_height, floor_line) = input
            .trim()
            .lines()
            .enumerate()
            .find(|(_, line)| line.trim().starts_with('1'))
            .unwrap();

        let total_stacks = floor_line
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks = std::iter::repeat(vec![])
            .take(total_stacks)
            .collect::<Vec<_>>();

        input
            .lines()
            .take(total_height)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .for_each(|line| {
                line.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|mut s| s.nth(1).unwrap())
                    .zip(stacks.iter_mut())
                    .filter(|(character, _)| *character != ' ')
                    .for_each(|(character, stack)| stack.push(character));
            });

        Self {
            stacks,
            instructions: input
                .trim()
                .lines()
                .skip(total_height + 2)
                .map(|line| {
                    let mut parts = line.split_whitespace();

                    Instruction {
                        amount: parts.nth(1).unwrap().parse().unwrap(),
                        source: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                        destination: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
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
