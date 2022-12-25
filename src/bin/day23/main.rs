use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/23/input.txt");

struct World {
    elves: HashSet<(i32, i32)>,
    round: usize,
}

fn n(coord: &(i32, i32)) -> (i32, i32) {
    (coord.0, coord.1 - 1)
}

fn s(coord: &(i32, i32)) -> (i32, i32) {
    (coord.0, coord.1 + 1)
}

fn e(coord: &(i32, i32)) -> (i32, i32) {
    (coord.0 + 1, coord.1)
}

fn w(coord: &(i32, i32)) -> (i32, i32) {
    (coord.0 - 1, coord.1)
}

fn ne(coord: &(i32, i32)) -> (i32, i32) {
    n(&e(coord))
}

fn nw(coord: &(i32, i32)) -> (i32, i32) {
    n(&w(coord))
}

fn se(coord: &(i32, i32)) -> (i32, i32) {
    s(&e(coord))
}

fn sw(coord: &(i32, i32)) -> (i32, i32) {
    s(&w(coord))
}

impl World {
    fn from_input(input: &str) -> Self {
        Self {
            elves: input
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, ch)| *ch == '#')
                        .map(|(x, _)| (x as i32, y as i32))
                        .collect::<Vec<_>>()
                })
                .collect(),
            round: 0,
        }
    }

    fn any_adjacent(&self, elf: &(i32, i32)) -> bool {
        [
            n(elf),
            s(elf),
            e(elf),
            w(elf),
            ne(elf),
            nw(elf),
            se(elf),
            sw(elf),
        ]
        .into_iter()
        .any(|coord| self.elves.contains(&coord))
    }

    fn check_n(&self, elf: &(i32, i32)) -> bool {
        [n(elf), ne(elf), nw(elf)]
            .into_iter()
            .all(|coord| !self.elves.contains(&coord))
    }

    fn check_s(&self, elf: &(i32, i32)) -> bool {
        [s(elf), se(elf), sw(elf)]
            .into_iter()
            .all(|coord| !self.elves.contains(&coord))
    }

    fn check_e(&self, elf: &(i32, i32)) -> bool {
        [e(elf), ne(elf), se(elf)]
            .into_iter()
            .all(|coord| !self.elves.contains(&coord))
    }

    fn check_w(&self, elf: &(i32, i32)) -> bool {
        [w(elf), nw(elf), sw(elf)]
            .into_iter()
            .all(|coord| !self.elves.contains(&coord))
    }

    fn next_round(&mut self) -> usize {
        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        self.elves
            .iter()
            .filter(|elf| self.any_adjacent(elf))
            .for_each(|elf| {
                if let Some(proposal) =
                    (0..4)
                        .into_iter()
                        .map(|n| (n + self.round) % 4)
                        .find_map(|proposal_number| match proposal_number {
                            0 => {
                                if self.check_n(elf) {
                                    Some(n(elf))
                                } else {
                                    None
                                }
                            }
                            1 => {
                                if self.check_s(elf) {
                                    Some(s(elf))
                                } else {
                                    None
                                }
                            }
                            2 => {
                                if self.check_w(elf) {
                                    Some(w(elf))
                                } else {
                                    None
                                }
                            }
                            3 => {
                                if self.check_e(elf) {
                                    Some(e(elf))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!("proposal number is between [0,3]"),
                        })
                {
                    proposals.entry(proposal).or_default().push(*elf);
                }
            });

        let mut count = 0;

        proposals
            .into_iter()
            .filter(|(_, elves)| elves.len() == 1)
            .for_each(|(coord, elves)| {
                self.elves.remove(&elves[0]);
                self.elves.insert(coord);
                count += 1;
            });

        self.round += 1;
        count
    }

    fn empty_tiles(&self) -> i32 {
        let width = self.elves.iter().map(|(x, _)| x).max().unwrap()
            - self.elves.iter().map(|(x, _)| x).min().unwrap()
            + 1;
        let height = self.elves.iter().map(|(_, y)| y).max().unwrap()
            - self.elves.iter().map(|(_, y)| y).min().unwrap()
            + 1;
        width * height - self.elves.len() as i32
    }
}

fn p1(input: &str) -> String {
    let mut world = World::from_input(input);

    (0..10).for_each(|_| {
        world.next_round();
    });

    world.empty_tiles().to_string()
}

fn p2(input: &str) -> String {
    let mut world = World::from_input(input);

    while world.next_round() > 0 {}

    world.round.to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./sample.txt");

    const SMALLER_EXAMPLE: &str = r"
.....
..##.
..#..
.....
..##.
.....
";
    #[test]
    fn test_parse() {
        let world = World::from_input(SMALLER_EXAMPLE);
        assert_eq!(
            world.elves,
            HashSet::from([(2, 1), (3, 1), (2, 2), (2, 4), (3, 4)])
        );
    }

    #[test]
    fn test_step() {
        let mut world = World::from_input(SMALLER_EXAMPLE);

        [
            r"
..##.
.....
..#..
...#.
..#..
.....
",
            r"
.....
..##.
.#...
....#
.....
..#..
",
            r"
..#..
....#
#....
....#
.....
..#..

",
            r"
..#..
....#
#....
....#
.....
..#..

",
        ]
        .into_iter()
        .enumerate()
        .for_each(|(index, expected)| {
            let expected_world = World::from_input(expected);

            world.next_round();
            assert_eq!(
                world.elves,
                expected_world.elves,
                "Round {} elves",
                index + 1
            );
            assert_eq!(world.round, index + 1, "Round {} round number", index + 1);
        });
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "110");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4049");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "20");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1021");
    }
}
