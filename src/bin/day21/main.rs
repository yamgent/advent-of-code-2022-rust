use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/21/input.txt");

enum Job<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Number(i64),
}

fn parse_graph(input: &str) -> HashMap<&str, Job> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (name, content) = line.split_once(": ").unwrap();

            let op = ["+", "-", "*", "/"]
                .into_iter()
                .find(|ch| content.contains(ch));

            let job = if let Some(op) = op {
                let (left, right) = content.split_once(op).unwrap();
                match op {
                    "+" => Job::Add(left.trim(), right.trim()),
                    "-" => Job::Sub(left.trim(), right.trim()),
                    "*" => Job::Mul(left.trim(), right.trim()),
                    "/" => Job::Div(left.trim(), right.trim()),
                    _ => unreachable!("Other ops not considered"),
                }
            } else {
                Job::Number(content.trim().parse().unwrap())
            };

            (name, job)
        })
        .collect()
}

fn p1(input: &str) -> String {
    let graph = parse_graph(input);

    fn traverse(graph: &HashMap<&str, Job>, name: &str) -> i64 {
        let job = graph.get(name).unwrap();

        match job {
            Job::Number(value) => *value,
            Job::Add(left, right)
            | Job::Sub(left, right)
            | Job::Mul(left, right)
            | Job::Div(left, right) => {
                let left = traverse(graph, left);
                let right = traverse(graph, right);
                match job {
                    Job::Add(..) => left + right,
                    Job::Sub(..) => left - right,
                    Job::Mul(..) => left * right,
                    Job::Div(..) => left / right,
                    _ => unreachable!("Already filtered"),
                }
            }
        }
    }

    traverse(&graph, "root").to_string()
}

fn p2(input: &str) -> String {
    let graph = parse_graph(input);

    fn calc_non_human(graph: &HashMap<&str, Job>, name: &str) -> Option<i64> {
        let job = graph.get(name).unwrap();

        match job {
            Job::Number(value) => {
                if name == "humn" {
                    None
                } else {
                    Some(*value)
                }
            }
            Job::Add(left, right)
            | Job::Sub(left, right)
            | Job::Mul(left, right)
            | Job::Div(left, right) => {
                if let Some(left) = calc_non_human(graph, left) {
                    if let Some(right) = calc_non_human(graph, right) {
                        return Some(match job {
                            Job::Add(..) => left + right,
                            Job::Sub(..) => left - right,
                            Job::Mul(..) => left * right,
                            Job::Div(..) => left / right,
                            _ => unreachable!("Already filtered"),
                        });
                    }
                }
                None
            }
        }
    }

    let (left, right) = match graph["root"] {
        Job::Add(left, right)
        | Job::Sub(left, right)
        | Job::Mul(left, right)
        | Job::Div(left, right) => (left, right),
        _ => panic!("Root should have two monkeys"),
    };

    let left_root = calc_non_human(&graph, left);
    let right_root = calc_non_human(&graph, right);

    fn propagate_to_humn(graph: &HashMap<&str, Job>, name: &str, acc: i64) -> i64 {
        let job = graph.get(name).unwrap();

        match job {
            Job::Number(..) => {
                if name == "humn" {
                    acc
                } else {
                    panic!("Should not process non-humn for this method.");
                }
            }
            Job::Add(left, right)
            | Job::Sub(left, right)
            | Job::Mul(left, right)
            | Job::Div(left, right) => {
                // TODO: Repeated computation
                let left_root = calc_non_human(graph, left);
                let right_root = calc_non_human(graph, right);

                if let Some(left_value) = left_root {
                    propagate_to_humn(
                        graph,
                        right,
                        match job {
                            Job::Add(..) => acc - left_value,
                            Job::Sub(..) => left_value - acc,
                            Job::Mul(..) => acc / left_value,
                            Job::Div(..) => left_value / acc,
                            _ => unreachable!("Already filtered"),
                        },
                    )
                } else if let Some(right_value) = right_root {
                    propagate_to_humn(
                        graph,
                        left,
                        match job {
                            Job::Add(..) => acc - right_value,
                            Job::Sub(..) => acc + right_value,
                            Job::Mul(..) => acc / right_value,
                            Job::Div(..) => acc * right_value,
                            _ => unreachable!("Already filtered"),
                        },
                    )
                } else {
                    panic!("Either inner side should give a concrete answer.");
                }
            }
        }
    }

    if let Some(left_value) = left_root {
        propagate_to_humn(&graph, right, left_value)
    } else if let Some(right_value) = right_root {
        propagate_to_humn(&graph, left, right_value)
    } else {
        panic!("Either side should give a concrete answer.");
    }
    .to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "152");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "72664227897438");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "301");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "3916491093817");
    }
}
