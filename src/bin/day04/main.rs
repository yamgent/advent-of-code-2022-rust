const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/04/input.txt");

struct Scenario {
    first_range: (i32, i32),
    second_range: (i32, i32),
}

impl Scenario {
    fn parse_line(line: &str) -> Self {
        fn parse_range(range: &str) -> (i32, i32) {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        }
        let (first_range, second_range) = line.split_once(',').unwrap();
        Scenario {
            first_range: parse_range(first_range),
            second_range: parse_range(second_range),
        }
    }
}

fn range_inside(a: &(i32, i32), b: &(i32, i32)) -> bool {
    (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
}

fn range_overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
    !(a.1 < b.0 || a.0 > b.1)
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Scenario::parse_line)
        .filter(|scenario| range_inside(&scenario.first_range, &scenario.second_range))
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Scenario::parse_line)
        .filter(|scenario| range_overlap(&scenario.first_range, &scenario.second_range))
        .count()
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
        assert_eq!(p1(SAMPLE_INPUT), "2");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "569");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "4");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "936");
    }
}
