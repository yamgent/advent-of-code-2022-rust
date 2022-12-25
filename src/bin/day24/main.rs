use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Blizzard {
    position: Coord,
    direction: Direction,
}

struct World {
    size: Coord,
    blizzards: Vec<Blizzard>,
    blizzards_pos_cache: HashSet<Coord>,
    start_position: Coord,
    end_position: Coord,
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blizzards_cache: HashMap<&Coord, Vec<Direction>> =
            self.blizzards
                .iter()
                .fold(HashMap::new(), |mut acc, entry| {
                    acc.entry(&entry.position)
                        .and_modify(|collection| collection.push(entry.direction))
                        .or_insert(vec![entry.direction]);
                    acc
                });

        writeln!(
            f,
            "{}.{}",
            std::iter::repeat('#')
                .take(self.start_position.x)
                .collect::<String>(),
            std::iter::repeat('#')
                .take(self.size.x - self.start_position.x - 1)
                .collect::<String>()
        )?;
        (1..(self.size.y - 1))
            .into_iter()
            .map(|y| {
                write!(f, "#")?;
                (1..(self.size.x - 1))
                    .into_iter()
                    .map(|x| match blizzards_cache.get(&Coord { x, y }) {
                        Some(collection) => match collection.len() {
                            1 => write!(
                                f,
                                "{}",
                                match collection[0] {
                                    Direction::Up => "^",
                                    Direction::Down => "v",
                                    Direction::Left => "<",
                                    Direction::Right => ">",
                                }
                            ),
                            _ => write!(f, "{}", collection.len()),
                        },
                        None => write!(f, "."),
                    })
                    .find(|res| res.is_err())
                    .unwrap_or(Ok(()))?;
                writeln!(f, "#")
            })
            .find(|res| res.is_err())
            .unwrap_or(Ok(()))?;
        writeln!(
            f,
            "{}.{}",
            std::iter::repeat('#')
                .take(self.end_position.x)
                .collect::<String>(),
            std::iter::repeat('#')
                .take(self.size.x - self.end_position.x - 1)
                .collect::<String>()
        )?;
        Ok(())
    }
}

impl World {
    fn from_input(input: &str) -> Self {
        let size = Coord {
            x: input.trim().lines().next().unwrap().len(),
            y: input.trim().lines().count(),
        };

        let blizzards = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| ch != &'.' && ch != &'#')
                    .map(|(x, ch)| Blizzard {
                        position: Coord { x, y },
                        direction: match ch {
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            _ => panic!("Illegal character {}", ch),
                        },
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let blizzards_pos_cache =
            HashSet::from_iter(blizzards.iter().map(|blizzard| blizzard.position));

        let start_position = Coord {
            x: input
                .trim()
                .lines()
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, ch)| ch == &'.')
                .unwrap()
                .0,
            y: 0,
        };

        let end_position = Coord {
            x: input
                .trim()
                .lines()
                .last()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, ch)| ch == &'.')
                .unwrap()
                .0,
            y: size.y - 1,
        };

        Self {
            size,
            blizzards,
            blizzards_pos_cache,
            start_position,
            end_position,
        }
    }

    fn step(&self) -> Self {
        let blizzards = self
            .blizzards
            .iter()
            .map(|blizzard| Blizzard {
                position: match blizzard.direction {
                    Direction::Up => Coord {
                        y: if blizzard.position.y == 1 {
                            self.size.y - 2
                        } else {
                            blizzard.position.y - 1
                        },
                        ..blizzard.position
                    },
                    Direction::Down => Coord {
                        y: if blizzard.position.y == self.size.y - 2 {
                            1
                        } else {
                            blizzard.position.y + 1
                        },
                        ..blizzard.position
                    },
                    Direction::Left => Coord {
                        x: if blizzard.position.x == 1 {
                            self.size.x - 2
                        } else {
                            blizzard.position.x - 1
                        },
                        ..blizzard.position
                    },
                    Direction::Right => Coord {
                        x: if blizzard.position.x == self.size.x - 2 {
                            1
                        } else {
                            blizzard.position.x + 1
                        },
                        ..blizzard.position
                    },
                },
                ..*blizzard
            })
            .collect::<Vec<_>>();

        let blizzards_pos_cache =
            HashSet::from_iter(blizzards.iter().map(|blizzard| blizzard.position));

        Self {
            blizzards,
            blizzards_pos_cache,
            ..*self
        }
    }
}

fn p1(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
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

    const QUESTION_EXAMPLE: &str = r"
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
";
    const MY_EXAMPLE: &str = r"
#.###
#.<.#
#..^#
##.##
";

    #[test]
    fn test_parse_input() {
        // question example
        assert_eq!(
            World::from_input(QUESTION_EXAMPLE).to_string().trim(),
            QUESTION_EXAMPLE.trim()
        );

        // my example
        assert_eq!(
            World::from_input(MY_EXAMPLE).to_string().trim(),
            MY_EXAMPLE.trim()
        );
    }

    #[test]
    fn test_step() {
        // question example
        let mut world = World::from_input(QUESTION_EXAMPLE);

        vec![
            r"
#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#
",
            r"
#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#
",
            r"
#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#
",
            r"
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#
",
            r"
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
",
        ]
        .into_iter()
        .for_each(|state| {
            world = world.step();

            assert_eq!(world.to_string().trim(), state.trim());
        });

        // my example
        let mut world = World::from_input(MY_EXAMPLE);
        vec![
            r"
#.###
#<.^#
#...#
##.##
",
            r"
#.###
#..<#
#..^#
##.##
",
            r"
#.###
#.<^#
#...#
##.##
",
            r"
#.###
#<..#
#..^#
##.##
",
            r"
#.###
#..2#
#...#
##.##
",
            r"
#.###
#.<.#
#..^#
##.##
",
        ]
        .into_iter()
        .for_each(|state| {
            world = world.step();

            assert_eq!(world.to_string().trim(), state.trim());
        });
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "");
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
