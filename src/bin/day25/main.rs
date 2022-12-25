use std::cmp::Ordering;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn from_snafu(value: &str) -> i64 {
    value.chars().fold(0, |acc, ch| {
        acc * 5
            + match ch {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid digit {}", ch),
            }
    })
}

fn to_snafu(mut value: i64) -> String {
    match value.cmp(&0) {
        Ordering::Less => unimplemented!("No idea how negative works."),
        Ordering::Equal => "0".to_string(),
        Ordering::Greater => {
            let mut stack = vec![];
            let mut carry = 0;

            while value > 0 {
                let current_digit = value % 5;
                value /= 5;

                let final_value = carry + current_digit;
                carry = (final_value >= 3).into();

                stack.push(match final_value {
                    0 | 1 | 2 => (final_value as u8 + b'0') as char,
                    3 => '=',
                    4 => '-',
                    5 => '0',
                    _ => unreachable!("Value cannot exceed [0-5]"),
                });
            }

            if carry == 1 {
                stack.push('1');
            }

            stack.iter().rev().collect()
        }
    }
}

fn p1(input: &str) -> String {
    to_snafu(input.trim().lines().map(from_snafu).sum())
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
        assert_eq!(p1(SAMPLE_INPUT), "2=-1=0");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "2-=12=2-2-2-=0012==2");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
