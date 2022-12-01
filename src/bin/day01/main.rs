const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn get_all_elves_calories(input: &str) -> Vec<i32> {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .sum::<i32>()
        })
        .collect()
}

fn p1(input: &str) -> String {
    get_all_elves_calories(input)
        .into_iter()
        .max()
        .unwrap()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut calories = get_all_elves_calories(input);
    calories.sort_unstable();
    calories.into_iter().rev().take(3).sum::<i32>().to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "24000");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "68292");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "45000");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "203203");
    }
}
