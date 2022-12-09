use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn normalize(value: i32) -> i32 {
    if value == 0 {
        0
    } else {
        value / value.abs()
    }
}

fn move_head(head: &(i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        "U" => (head.0, head.1 - 1),
        "D" => (head.0, head.1 + 1),
        _ => panic!("Unknown direction {}", dir),
    }
}

fn update_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        tail.clone()
    } else {
        let diff = (normalize(head.0 - tail.0), normalize(head.1 - tail.1));
        (tail.0 + diff.0, tail.1 + diff.1)
    }
}

fn p1(input: &str) -> String {
    let mut visited = HashSet::from([(0, 0)]);
    let mut head = (0, 0);
    let mut tail = (0, 0);

    input.trim().lines().for_each(|line| {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        (0..count).into_iter().for_each(|_| {
            head = move_head(&head, dir);
            tail = update_tail(&head, &tail);
            visited.insert(tail);
        });
    });
    visited.len().to_string()
}

fn p2(input: &str) -> String {
    let mut visited = HashSet::from([(0, 0)]);
    let mut body = std::iter::repeat((0, 0)).take(10).collect::<Vec<_>>();

    input.trim().lines().for_each(|line| {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();

        (0..count).into_iter().for_each(|_| {
            let head = move_head(&body[0], dir);
            body = body.iter().skip(1).fold(vec![head], |mut acc, body_part| {
                let body_part = update_tail(acc.iter().last().unwrap(), body_part);
                acc.push(body_part);
                acc
            });
            visited.insert(*body.iter().last().unwrap());
        });
    });
    visited.len().to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "1");
        assert_eq!(
            p2(r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"),
            "36"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "2511");
    }
}
