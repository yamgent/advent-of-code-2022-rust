use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[allow(dead_code)]
fn solve_naive(input: &str, distinct_count: usize) -> String {
    (input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .windows(distinct_count)
        .enumerate()
        .find(|(_, values)| {
            (0..values.len()).all(|i| ((i + 1)..values.len()).all(|j| values[i] != values[j]))
        })
        .unwrap()
        .0
        + distinct_count)
        .to_string()
}

fn solve(input: &str, distinct_count: usize) -> String {
    let mut last_seen = HashMap::new();
    let mut start = 0usize;

    (input
        .trim()
        .chars()
        .enumerate()
        .find(|(index, ch)| {
            if let Some(pos) = last_seen.get(ch) {
                if *pos >= start {
                    start = *pos + 1;
                }
            }
            last_seen.insert(*ch, *index);
            index - start + 1 == distinct_count
        })
        .unwrap()
        .0
        + 1)
    .to_string()
}

fn p1(input: &str) -> String {
    solve(input, 4)
}

fn p2(input: &str) -> String {
    solve(input, 14)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1287");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(p2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "3716");
    }
}
