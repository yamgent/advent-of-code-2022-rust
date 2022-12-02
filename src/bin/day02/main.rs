const ACTUAL_INPUT: &str = include_str!("./input.txt");

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(shape: &str) -> Self {
        match shape {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("{} is not a valid shape", shape),
        }
    }
}

struct Play(Shape, Shape);

impl Play {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(" ").map(Shape::parse);

        let opponent = parts
            .next()
            .unwrap_or_else(|| panic!("Expected two parts, found nothing for {}", line));
        let me = parts
            .next()
            .unwrap_or_else(|| panic!("Expected two parts, only found one for {}", line));
        if parts.count() > 0 {
            panic!("Expected two parts, found more than two for {}", line);
        }

        Self(opponent, me)
    }
}

enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Outcome {
    fn get_outcome(play: &Play) -> Self {
        match play {
            Play(Shape::Rock, Shape::Paper)
            | Play(Shape::Paper, Shape::Scissors)
            | Play(Shape::Scissors, Shape::Rock) => Outcome::Win,
            Play(Shape::Rock, Shape::Rock)
            | Play(Shape::Paper, Shape::Paper)
            | Play(Shape::Scissors, Shape::Scissors) => Outcome::Tie,
            _ => Outcome::Lose,
        }
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Play::parse)
        .map(|play| {
            let outcome_score = match Outcome::get_outcome(&play) {
                Outcome::Win => 6,
                Outcome::Tie => 3,
                Outcome::Lose => 0,
            };
            let shape_score = match &play.1 {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            outcome_score + shape_score
        })
        .sum::<i32>()
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
        assert_eq!(p1(SAMPLE_INPUT), "15");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "10595");
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
