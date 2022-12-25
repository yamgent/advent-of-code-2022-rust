use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

struct Universe {
    worlds: RefCell<Vec<World>>,
    size: Coord,
    start_position: Coord,
    end_position: Coord,
}

impl Universe {
    fn from_input(input: &str) -> Self {
        let world = World::from_input(input);
        Self {
            size: world.size,
            start_position: world.start_position,
            end_position: world.end_position,
            worlds: RefCell::new(vec![world]),
        }
    }

    fn simulate_universe_until(&self, time: usize) {
        while self.worlds.borrow().len() < time + 1 {
            let next_world = self.worlds.borrow().iter().last().unwrap().step();
            self.worlds.borrow_mut().push(next_world);
        }
    }

    fn is_valid_coord_at_time(&self, coord: &Coord, time: usize) -> bool {
        if coord == &self.start_position || coord == &self.end_position {
            return true;
        }

        if coord.x <= 0 || coord.x >= self.size.x - 1 || coord.y <= 0 || coord.y >= self.size.y - 1
        {
            return false;
        }

        self.simulate_universe_until(time);

        !self
            .worlds
            .borrow()
            .get(time)
            .unwrap()
            .blizzards_pos_cache
            .contains(&coord)
    }

    fn next_steps(&self, coord: &Coord, current_time: usize) -> Vec<Coord> {
        vec![
            Coord {
                x: coord.x - 1,
                ..*coord
            },
            Coord {
                x: coord.x + 1,
                ..*coord
            },
            Coord {
                // underflow can happen at start_position
                y: if coord.y == 0 { 0 } else { coord.y - 1 },
                ..*coord
            },
            Coord {
                y: coord.y + 1,
                ..*coord
            },
            // waiting is a valid move
            *coord,
        ]
        .into_iter()
        .filter(|coord| self.is_valid_coord_at_time(coord, current_time + 1))
        .collect()
    }
}

fn find_shortest(
    universe: &Universe,
    start_time: usize,
    source: Coord,
    destination: Coord,
) -> usize {
    let mut time = start_time;
    let mut queue = vec![source];

    while !queue.is_empty() {
        let mut visited: HashSet<Coord> = HashSet::new();

        if queue.iter().any(|coord| coord == &destination) {
            return time;
        }

        queue = queue
            .into_iter()
            .flat_map(|coord| {
                let next_steps = universe
                    .next_steps(&coord, time)
                    .into_iter()
                    .filter(|coord| !visited.contains(coord))
                    .collect::<Vec<_>>();

                visited.extend(next_steps.clone().iter());
                next_steps
            })
            .collect();

        time += 1;
    }

    unreachable!("We never run out of moves, we only have too much moves.")
}

fn p1(input: &str) -> String {
    let universe = Universe::from_input(&input);

    find_shortest(&universe, 0, universe.start_position, universe.end_position).to_string()
}

fn p2(input: &str) -> String {
    let universe = Universe::from_input(&input);

    let first = find_shortest(&universe, 0, universe.start_position, universe.end_position);

    let second = find_shortest(
        &universe,
        first,
        universe.end_position,
        universe.start_position,
    );
    find_shortest(
        &universe,
        second,
        universe.start_position,
        universe.end_position,
    )
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
    fn test_universe() {
        let universe = Universe::from_input(SAMPLE_INPUT);
        universe.simulate_universe_until(18);
        assert_eq!(
            universe.worlds.borrow().get(18).unwrap().to_string().trim(),
            r"
#.######
#>2.<.<#
#.2v^2<#
#>..>2>#
#<....>#
######.#
"
            .trim()
        );
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "18");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "228");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "54");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "723");
    }
}
