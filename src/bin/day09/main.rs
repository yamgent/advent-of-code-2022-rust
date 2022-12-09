use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn normalize(value: i32) -> i32 {
    if value == 0 {
        0
    } else {
        value / value.abs()
    }
}

fn update_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return;
    }
    let diff = (normalize(head.0 - tail.0), normalize(head.1 - tail.1));
    *tail = (tail.0 + diff.0, tail.1 + diff.1);
}

fn p1(input: &str) -> String {
    let mut visited = HashSet::from([(0, 0)]);
    let mut head = (0, 0);
    let mut tail = (0, 0);

    input.trim().lines().for_each(|line| {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        (0..count).into_iter().for_each(|_| {
            match dir {
                "L" => {
                    head.0 -= 1;
                }
                "R" => {
                    head.0 += 1;
                }
                "U" => {
                    head.1 -= 1;
                }
                "D" => {
                    head.1 += 1;
                }
                _ => panic!("Unknown direction {}", dir),
            }

            update_tail(&head, &mut tail);
            visited.insert(tail);
        });
    });
    visited.len().to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "13");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "6332");
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
