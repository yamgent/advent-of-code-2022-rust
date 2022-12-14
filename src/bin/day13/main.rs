use std::cmp::Ordering;

use serde_json::Value;

const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn determine_order(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            left.as_i64().unwrap().cmp(&right.as_i64().unwrap())
        }
        (Value::Array(left), Value::Array(right)) => {
            if let Some(result) = left.iter().zip(right.iter()).find_map(|(left, right)| {
                let result = determine_order(left, right);
                if matches!(result, Ordering::Equal) {
                    None
                } else {
                    Some(result)
                }
            }) {
                result
            } else {
                left.len().cmp(&right.len())
            }
        }
        (Value::Number(..), Value::Array(..)) => {
            determine_order(&Value::Array(vec![left.clone()]), right)
        }
        (Value::Array(..), Value::Number(..)) => {
            determine_order(left, &Value::Array(vec![right.clone()]))
        }
        _ => {
            panic!("Combination is not supported.");
        }
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter(|(_, line)| {
            let (left, right) = line.split_once('\n').unwrap();
            let (left, right): (Value, Value) = (
                serde_json::from_str(left).unwrap(),
                serde_json::from_str(right).unwrap(),
            );
            matches!(determine_order(&left, &right), Ordering::Less)
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let mut packets = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();

    packets.extend([
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ]);
    packets.sort_by(determine_order);

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| *packet.to_string() == *"[[2]]" || *packet.to_string() == *"[[6]]")
        .map(|(index, _)| index + 1)
        .product::<usize>()
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
        assert_eq!(p1(SAMPLE_INPUT), "13");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "5503");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "140");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "20952");
    }
}
