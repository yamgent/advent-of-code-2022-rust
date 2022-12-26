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
        match graph.get(name).unwrap() {
            Job::Number(value) => *value,
            Job::Add(left, right) => traverse(graph, left) + traverse(graph, right),
            Job::Sub(left, right) => traverse(graph, left) - traverse(graph, right),
            Job::Mul(left, right) => traverse(graph, left) * traverse(graph, right),
            Job::Div(left, right) => traverse(graph, left) / traverse(graph, right),
        }
    }

    traverse(&graph, "root").to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "152");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "72664227897438");
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
