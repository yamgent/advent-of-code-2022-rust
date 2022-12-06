const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn p1(input: &str) -> String {
    (input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(_, values)| {
            (0..values.len()).all(|i| ((i + 1)..values.len()).all(|j| values[i] != values[j]))
        })
        .unwrap()
        .0
        + 4)
    .to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
