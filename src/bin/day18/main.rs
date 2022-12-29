use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/18/input.txt");

fn p1(input: &str) -> String {
    let mut seen = HashMap::new();

    fn mark(
        seen: &mut HashMap<(i32, i32, i32, i32, i32, i32), usize>,
        key: (i32, i32, i32, i32, i32, i32),
    ) {
        *seen.entry(key).or_insert(0) += 1;
    }

    input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|cube| {
            mark(
                &mut seen,
                (cube[0], cube[0], cube[1], cube[1] + 1, cube[2], cube[2] + 1),
            );
            mark(
                &mut seen,
                (
                    cube[0] + 1,
                    cube[0] + 1,
                    cube[1],
                    cube[1] + 1,
                    cube[2],
                    cube[2] + 1,
                ),
            );
            mark(
                &mut seen,
                (cube[0], cube[0] + 1, cube[1], cube[1], cube[2], cube[2] + 1),
            );
            mark(
                &mut seen,
                (
                    cube[0],
                    cube[0] + 1,
                    cube[1] + 1,
                    cube[1] + 1,
                    cube[2],
                    cube[2] + 1,
                ),
            );
            mark(
                &mut seen,
                (cube[0], cube[0] + 1, cube[1], cube[1] + 1, cube[2], cube[2]),
            );
            mark(
                &mut seen,
                (
                    cube[0],
                    cube[0] + 1,
                    cube[1],
                    cube[1] + 1,
                    cube[2] + 1,
                    cube[2] + 1,
                ),
            );
        });

    seen.into_iter()
        .filter(|(_, count)| *count == 1)
        .count()
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
        assert_eq!(p1(SAMPLE_INPUT), "64");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4244");
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
