const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/22/input.txt");

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Forward(i32),
    Left,
    Right,
}

impl Instruction {
    fn parse(line: &str) -> Vec<Self> {
        let mut insts = vec![];

        let mut number = 0;

        line.chars().for_each(|ch| match ch {
            'L' | 'R' => {
                if number > 0 {
                    insts.push(Instruction::Forward(number));
                    number = 0;
                }
                insts.push(match ch {
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => unreachable!("Not possible"),
                });
            }
            '0'..='9' => {
                number *= 10;
                number += (ch as u8 - b'0') as i32;
            }
            _ => panic!("Unknown char {}", ch),
        });

        if number > 0 {
            insts.push(Instruction::Forward(number));
        }

        insts
    }
}

struct FloorPlan {
    tiles: Vec<Vec<char>>,
    minmax_row: Vec<(usize, usize)>,
    minmax_col: Vec<(usize, usize)>,
}

impl FloorPlan {
    fn parse(lines: &str) -> Self {
        let tiles = lines
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let minmax_row = tiles
            .iter()
            .map(|row| {
                (
                    row.iter()
                        .enumerate()
                        .find(|(_, ch)| **ch != ' ')
                        .map(|(idx, _)| idx)
                        .unwrap_or_default(),
                    row.iter()
                        .enumerate()
                        .rev()
                        .find(|(_, ch)| **ch != ' ')
                        .map(|(idx, _)| idx)
                        .unwrap_or_default(),
                )
            })
            .collect();

        let total_col = tiles.iter().map(|row| row.len()).max().unwrap();

        let minmax_col = (0..total_col)
            .into_iter()
            .map(|col_idx| {
                (
                    tiles
                        .iter()
                        .enumerate()
                        .find(|(_, row)| col_idx < row.len() && row[col_idx] != ' ')
                        .map(|(idx, _)| idx)
                        .unwrap_or_default(),
                    tiles
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, row)| col_idx < row.len() && row[col_idx] != ' ')
                        .map(|(idx, _)| idx)
                        .unwrap_or_default(),
                )
            })
            .collect();

        Self {
            tiles,
            minmax_row,
            minmax_col,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Person {
    position: (usize, usize),
    facing: usize,
}

impl Person {
    fn password(&self) -> String {
        ((self.position.1 + 1) * 1000 + (self.position.0 + 1) * 4 + self.facing).to_string()
    }
}

impl Instruction {
    fn execute(&self, floor_plan: &FloorPlan, person: &Person) -> Person {
        match self {
            Instruction::Left => Person {
                facing: if person.facing == 0 {
                    3
                } else {
                    person.facing - 1
                },
                ..*person
            },
            Instruction::Right => Person {
                facing: (person.facing + 1) % 4,
                ..*person
            },
            Instruction::Forward(steps) => {
                let mut current = person.position;

                (0..(*steps as usize)).into_iter().find(|_| {
                    let next = match person.facing {
                        0 => (
                            if current.0 == floor_plan.minmax_row[current.1].1 {
                                floor_plan.minmax_row[current.1].0
                            } else {
                                current.0 + 1
                            },
                            current.1,
                        ),
                        1 => (
                            current.0,
                            if current.1 == floor_plan.minmax_col[current.0].1 {
                                floor_plan.minmax_col[current.0].0
                            } else {
                                current.1 + 1
                            },
                        ),
                        2 => (
                            if current.0 == floor_plan.minmax_row[current.1].0 {
                                floor_plan.minmax_row[current.1].1
                            } else {
                                current.0 - 1
                            },
                            current.1,
                        ),
                        3 => (
                            current.0,
                            if current.1 == floor_plan.minmax_col[current.0].0 {
                                floor_plan.minmax_col[current.0].1
                            } else {
                                current.1 - 1
                            },
                        ),
                        _ => unreachable!("Not a valid facing."),
                    };

                    let is_wall = floor_plan.tiles[next.1][next.0] == '#';
                    if !is_wall {
                        current = next;
                    }
                    is_wall
                });

                Person {
                    position: current,
                    ..*person
                }
            }
        }
    }
}

fn p1(input: &str) -> String {
    let (floor_plan, instructions) = input.split_once("\n\n").unwrap();
    let floor_plan = FloorPlan::parse(floor_plan);
    let instructions = Instruction::parse(instructions.trim());

    instructions
        .into_iter()
        .fold(
            Person {
                position: (floor_plan.minmax_row[0].0, 0),
                facing: 0,
            },
            |person, inst| inst.execute(&floor_plan, &person),
        )
        .password()
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
    fn test_parse_instructions() {
        assert_eq!(
            Instruction::parse("10R5L5R10L4R5L5"),
            vec![
                Instruction::Forward(10),
                Instruction::Right,
                Instruction::Forward(5),
                Instruction::Left,
                Instruction::Forward(5),
                Instruction::Right,
                Instruction::Forward(10),
                Instruction::Left,
                Instruction::Forward(4),
                Instruction::Right,
                Instruction::Forward(5),
                Instruction::Left,
                Instruction::Forward(5),
            ]
        );
    }

    #[test]
    fn test_parse_floorplan() {
        let floor_plan = FloorPlan::parse(
            r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.",
        );
        assert_eq!(
            floor_plan.minmax_row,
            vec![
                (8, 11),
                (8, 11),
                (8, 11),
                (8, 11),
                (0, 11),
                (0, 11),
                (0, 11),
                (0, 11),
                (8, 15),
                (8, 15),
                (8, 15),
                (8, 15),
            ]
        );
        assert_eq!(
            floor_plan.minmax_col,
            vec![
                (4, 7),
                (4, 7),
                (4, 7),
                (4, 7),
                (4, 7),
                (4, 7),
                (4, 7),
                (4, 7),
                (0, 11),
                (0, 11),
                (0, 11),
                (0, 11),
                (8, 11),
                (8, 11),
                (8, 11),
                (8, 11),
            ]
        );
    }

    #[test]
    fn test_steps_horizontal() {
        let floor = FloorPlan::parse(".....");
        let insts = Instruction::parse("RL");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 0,
                }
            ),
            Person {
                position: (0, 0),
                facing: 1
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 3,
                }
            ),
            Person {
                position: (0, 0),
                facing: 0
            }
        );
        assert_eq!(
            insts[1].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 0,
                }
            ),
            Person {
                position: (0, 0),
                facing: 3
            }
        );
        assert_eq!(
            insts[1].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 3,
                }
            ),
            Person {
                position: (0, 0),
                facing: 2
            }
        );

        let insts = Instruction::parse("2");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 0
                }
            ),
            Person {
                position: (2, 0),
                facing: 0
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (4, 0),
                    facing: 0
                }
            ),
            Person {
                position: (1, 0),
                facing: 0
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 2
                }
            ),
            Person {
                position: (3, 0),
                facing: 2
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (4, 0),
                    facing: 2
                }
            ),
            Person {
                position: (2, 0),
                facing: 2
            }
        );

        let floor = FloorPlan::parse("#..#...");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (1, 0),
                    facing: 0
                }
            ),
            Person {
                position: (2, 0),
                facing: 0
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (2, 0),
                    facing: 0
                }
            ),
            Person {
                position: (2, 0),
                facing: 0
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (6, 0),
                    facing: 0
                }
            ),
            Person {
                position: (6, 0),
                facing: 0
            }
        );

        let floor = FloorPlan::parse("...#..#");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (5, 0),
                    facing: 2
                }
            ),
            Person {
                position: (4, 0),
                facing: 2
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (4, 0),
                    facing: 2
                }
            ),
            Person {
                position: (4, 0),
                facing: 2
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 2
                }
            ),
            Person {
                position: (0, 0),
                facing: 2
            }
        );
    }

    #[test]
    fn test_steps_vertical() {
        let floor = FloorPlan::parse(".\n.\n.\n.\n.");
        let insts = Instruction::parse("2");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 1
                }
            ),
            Person {
                position: (0, 2),
                facing: 1
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 4),
                    facing: 1
                }
            ),
            Person {
                position: (0, 1),
                facing: 1
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 3
                }
            ),
            Person {
                position: (0, 3),
                facing: 3
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 4),
                    facing: 3
                }
            ),
            Person {
                position: (0, 2),
                facing: 3
            }
        );

        // TODO: change

        let floor = FloorPlan::parse("#\n.\n.\n#\n.\n.\n.");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 1),
                    facing: 1
                }
            ),
            Person {
                position: (0, 2),
                facing: 1
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 2),
                    facing: 1
                }
            ),
            Person {
                position: (0, 2),
                facing: 1
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 6),
                    facing: 1
                }
            ),
            Person {
                position: (0, 6),
                facing: 1
            }
        );

        let floor = FloorPlan::parse(".\n.\n.\n#\n.\n.\n#");
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 5),
                    facing: 3
                }
            ),
            Person {
                position: (0, 4),
                facing: 3
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 4),
                    facing: 3
                }
            ),
            Person {
                position: (0, 4),
                facing: 3
            }
        );
        assert_eq!(
            insts[0].execute(
                &floor,
                &Person {
                    position: (0, 0),
                    facing: 3
                }
            ),
            Person {
                position: (0, 0),
                facing: 3
            }
        );
    }

    #[test]
    fn test_password() {
        assert_eq!(
            Person {
                position: (7, 5),
                facing: 0
            }
            .password(),
            "6032"
        );
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "6032");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "97356");
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
