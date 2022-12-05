const ACTUAL_INPUT: &str = include_str!("./input.txt");
const ACTUAL_META: (usize, usize) = (9, 8);

fn p1(input: &str, meta: &(usize, usize)) -> String {
    let mut stacks = std::iter::repeat(vec![]).take(meta.0).collect::<Vec<_>>();

    input
        .lines()
        .take(meta.1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .for_each(|line| {
            let mut line = line.chars();
            stacks.iter_mut().for_each(|stack| {
                let character = line.nth(1).unwrap();
                line.next();
                line.next();

                if character != ' ' {
                    stack.push(character);
                }
            });
        });

    input
        .trim()
        .lines()
        .skip(meta.1 + 2)
        .map(|line| {
            let mut parts = line.split_whitespace();
            let amount = parts.nth(1).unwrap().parse().unwrap();
            let source = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let destination = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            (amount, source, destination)
        })
        .for_each(|(amount, source, destination)| {
            (0..amount).for_each(|_| {
                let value = stacks[source].pop().unwrap();
                stacks[destination].push(value);
            });
        });

    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn p2(input: &str, meta: &(usize, usize)) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT, &ACTUAL_META));
    println!("{}", p2(ACTUAL_INPUT, &ACTUAL_META));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./sample.txt");
    const SAMPLE_META: (usize, usize) = (3, 3);

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT, &SAMPLE_META), "CMZ");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT, &ACTUAL_META), "GFTNRBZPF");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT, &SAMPLE_META), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT, &ACTUAL_META), "");
    }
}
