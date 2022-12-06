use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[allow(dead_code)]
fn solve_naive(input: &str, distinct_count: usize) -> String {
    (input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .windows(distinct_count)
        .position(|values| {
            (0..values.len()).all(|i| ((i + 1)..values.len()).all(|j| values[i] != values[j]))
        })
        .unwrap()
        + distinct_count)
        .to_string()
}

#[allow(dead_code)]
fn solve_improved(input: &str, distinct_count: usize) -> String {
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

#[allow(dead_code)]
fn solve_bitset(input: &str, distinct_count: usize) -> String {
    // uses bitset to check distinct
    // from: https://www.reddit.com/r/adventofcode/comments/zdw0u6/comment/iz4lb8u/?utm_source=reddit&utm_medium=web2x&context=3
    (input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .windows(distinct_count)
        .position(|w| {
            w.iter()
                .fold(0u32, |acc, ch| acc | 1 << (*ch as u32 - 'a' as u32))
                .count_ones() as usize
                == distinct_count
        })
        .unwrap()
        + distinct_count)
        .to_string()
}

fn solve(input: &str, distinct_count: usize) -> String {
    // use "rolling" mask
    fn toggle(acc: u32, ch: char) -> u32 {
        acc ^ (1 << (ch as u32 - 'a' as u32))
    }
    fn toggle2(acc: u32, ch1: char, ch2: char) -> u32 {
        toggle(toggle(acc, ch1), ch2)
    }
    fn all_distinct(mask: u32, distinct_count: usize) -> bool {
        mask.count_ones() as usize == distinct_count
    }

    let mut mask = input.trim().chars().take(distinct_count).fold(0u32, toggle);

    if all_distinct(mask, distinct_count) {
        distinct_count.to_string()
    } else {
        (distinct_count
            + 1
            + input
                .trim()
                .chars()
                .zip(input.trim().chars().skip(distinct_count))
                .position(|(old, new)| {
                    mask = toggle2(mask, old, new);
                    all_distinct(mask, distinct_count)
                })
                .unwrap())
        .to_string()
    }
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
