const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
enum Operand {
    Val(i64),
    Old,
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

impl Operation {
    fn compute(&self, val: &i64) -> i64 {
        match self {
            Operation::Add(operand) => match operand {
                Operand::Val(other) => val + other,
                Operand::Old => val + val,
            },
            Operation::Mul(operand) => match operand {
                Operand::Val(other) => val * other,
                Operand::Old => val * val,
            },
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: i64,
    throw_true: usize,
    throw_false: usize,
    inspected: usize,
}

impl Monkey {
    fn parse_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .split("\n\n")
            .map(|subinput| {
                let mut lines = subinput.trim().lines().skip(1);
                let items = lines
                    .next()
                    .unwrap()
                    .split_once(':')
                    .unwrap()
                    .1
                    .trim()
                    .split(',')
                    .map(|val| val.trim().parse().unwrap())
                    .collect();

                let (operator, operand) = lines
                    .next()
                    .unwrap()
                    .split_once("= old ")
                    .unwrap()
                    .1
                    .trim()
                    .split_once(' ')
                    .unwrap();
                let operand = if operand == "old" {
                    Operand::Old
                } else {
                    Operand::Val(operand.parse().unwrap())
                };
                let op = match operator {
                    "+" => Operation::Add(operand),
                    "*" => Operation::Mul(operand),
                    _ => panic!("Unknown operator {}", operator),
                };

                let test = lines
                    .next()
                    .unwrap()
                    .split_once("by")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                let throw_true = lines
                    .next()
                    .unwrap()
                    .split_once("monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                let throw_false = lines
                    .next()
                    .unwrap()
                    .split_once("monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                Self {
                    items,
                    op,
                    test,
                    throw_true,
                    throw_false,
                    inspected: 0,
                }
            })
            .collect()
    }
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, worry: impl Fn(i64) -> i64) -> String
where
{
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            let monkey = &mut monkeys[i];
            let destinations = monkey
                .items
                .iter()
                .map(|val| {
                    let val = worry(monkey.op.compute(val));
                    let target = if val % monkey.test == 0 {
                        monkey.throw_true
                    } else {
                        monkey.throw_false
                    };
                    (target, val)
                })
                .collect::<Vec<_>>();
            monkey.inspected += monkey.items.len();
            monkey.items.clear();

            destinations.into_iter().for_each(|(target, val)| {
                monkeys[target].items.push(val);
            });
        });
    });

    monkeys.sort_by(|a, b| a.inspected.cmp(&b.inspected).reverse());
    monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.inspected)
        .product::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    let monkeys = Monkey::parse_input(input);
    solve(monkeys, 20, |val| val / 3)
}

fn p2(input: &str) -> String {
    let monkeys = Monkey::parse_input(input);
    let prime = monkeys.iter().map(|monkey| monkey.test).product::<i64>();
    solve(monkeys, 10000, |val| val % prime)
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
        assert_eq!(p1(SAMPLE_INPUT), "10605");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "99852");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "2713310158");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "25935263541");
    }
}
