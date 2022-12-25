use itertools::Itertools;
use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/14/input.txt");

#[derive(Debug)]
struct World {
    rocks: HashSet<(i32, i32)>,
    abyss: i32,
    original_rocks_count: usize,
}

impl World {
    fn parse_input(input: &str) -> Self {
        let rocks = HashSet::from_iter(input.trim().lines().flat_map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (a, b) = point.split_once(',').unwrap();
                    (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
                })
                .tuple_windows()
                .flat_map(|(a, b)| {
                    if a.0 == b.0 {
                        ((a.1.min(b.1))..=(a.1.max(b.1)))
                            .into_iter()
                            .map(|y| (a.0, y))
                            .collect::<Vec<_>>()
                    } else if a.1 == b.1 {
                        ((a.0.min(b.0))..=(a.0.max(b.0)))
                            .into_iter()
                            .map(|x| (x, a.1))
                            .collect::<Vec<_>>()
                    } else {
                        panic!("{:?} to {:?} is not supported", a, b);
                    }
                })
        }));
        let abyss = rocks.iter().map(|point| point.1).max().unwrap() + 1;
        let original_rocks_count = rocks.len();

        Self {
            rocks,
            abyss,
            original_rocks_count,
        }
    }

    fn add_sand(&mut self) -> bool {
        let mut position = (500, 0);

        if self.rocks.contains(&position) {
            return false;
        }

        while position.1 < self.abyss {
            if let Some(pos) = [
                (position.0, position.1 + 1),
                (position.0 - 1, position.1 + 1),
                (position.0 + 1, position.1 + 1),
            ]
            .into_iter()
            .find(|pos| !self.rocks.contains(pos))
            {
                position = pos;
            } else {
                break;
            }
        }

        if position.1 < self.abyss {
            self.rocks.insert(position);
            true
        } else {
            false
        }
    }

    fn add_sand_p2(&mut self) -> bool {
        let mut position = (500, 0);

        if self.rocks.contains(&position) {
            return false;
        }

        while position.1 < self.abyss {
            if let Some(pos) = [
                (position.0, position.1 + 1),
                (position.0 - 1, position.1 + 1),
                (position.0 + 1, position.1 + 1),
            ]
            .into_iter()
            .find(|pos| !self.rocks.contains(pos))
            {
                position = pos;
            } else {
                break;
            }
        }

        self.rocks.insert(position);
        true
    }

    fn get_total_sands(&self) -> i32 {
        self.rocks.len() as i32 - self.original_rocks_count as i32
    }
}

fn p1(input: &str) -> String {
    let mut world = World::parse_input(input);
    while world.add_sand() {}
    world.get_total_sands().to_string()
}

fn p2(input: &str) -> String {
    let mut world = World::parse_input(input);
    while world.add_sand_p2() {}
    world.get_total_sands().to_string()
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
        assert_eq!(p1(SAMPLE_INPUT), "24");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "832");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "93");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "27601");
    }
}
