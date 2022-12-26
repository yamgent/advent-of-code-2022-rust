const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/20/input.txt");

struct Node {
    value: i64,
    next: usize,
    prev: usize,
}

fn parse_input(input: &str, decryption_key: i64) -> Vec<Node> {
    let mut result = input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .map(|(idx, value)| Node {
            value: value * decryption_key,
            next: idx + 1,
            prev: if idx == 0 { 0 } else { idx - 1 },
        })
        .collect::<Vec<_>>();

    result.get_mut(0).unwrap().prev = result.len() - 1;
    result.iter_mut().rev().next().unwrap().next = 0;

    result
}

fn find_zero_value_pos(list: &Vec<Node>) -> usize {
    list.iter()
        .enumerate()
        .find(|(_, node)| node.value == 0)
        .unwrap()
        .0
}

fn find_nth_value_from_zero(list: &Vec<Node>, nth: usize) -> i64 {
    let nth = nth % list.len();
    let mut cur = find_zero_value_pos(list);
    for _ in 0..nth {
        cur = list[cur].next;
    }
    list[cur].value
}

// for debugging purposes
#[allow(dead_code)]
fn print_from_zero(list: &Vec<Node>) {
    let zero_value_node_pos = find_zero_value_pos(list);

    print!("0");

    let mut cur = list[zero_value_node_pos].next;
    while list[cur].value != 0 {
        print!(" {}", list[cur].value);
        cur = list[cur].next;
    }
    println!();
}

fn decrypt(input: &str, decryption_key: i64, mix_count: usize) -> i64 {
    let mut list = parse_input(input, decryption_key);

    for _ in 0..mix_count {
        for i in 0..(list.len()) {
            if list[i].value == 0 {
                continue;
            }

            // remove ith node
            let prev = list[i].prev;
            let next = list[i].next;
            list[prev].next = next;
            list[next].prev = prev;

            // find the new spot position
            let mut cur = prev;
            if list[i].value > 0 {
                let steps = (list[i].value as usize) % (list.len() - 1);
                for _ in 0..steps {
                    cur = list[cur].next;
                }
            } else {
                let steps = (-list[i].value as usize) % (list.len() - 1);
                for _ in 0..steps {
                    cur = list[cur].prev;
                }
            }

            // insert ith node after finding its correct position
            let next = list[cur].next;
            list[cur].next = i;
            list[i].prev = cur;
            list[next].prev = i;
            list[i].next = next;
        }
    }

    find_nth_value_from_zero(&list, 1000)
        + find_nth_value_from_zero(&list, 2000)
        + find_nth_value_from_zero(&list, 3000)
}

fn p1(input: &str) -> String {
    decrypt(input, 1, 1).to_string()
}

fn p2(input: &str) -> String {
    decrypt(input, 811589153, 10).to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "3");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "10763");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "1623178306");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "4979911042808");
    }
}
