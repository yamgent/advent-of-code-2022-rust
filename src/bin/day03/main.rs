use itertools::Itertools;
use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn get_priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => panic!("{} does not have a priority", ch),
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.chars().collect::<HashSet<_>>();
            right.chars().find(|ch| left.contains(ch)).unwrap()
        })
        .map(get_priority)
        .sum::<u32>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| {
            *a.intersection(&b)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .exactly_one()
                .unwrap()
        })
        .map(get_priority)
        .sum::<u32>()
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
        assert_eq!(p1(SAMPLE_INPUT), "157");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "7691");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "70");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2508");
    }
}
