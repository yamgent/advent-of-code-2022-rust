use std::collections::{BinaryHeap, HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct World {
    start: (usize, usize),
    end: (usize, usize),
    heights: Vec<Vec<i32>>,
}

enum SourceToConsider {
    OnlyStart,
    AllLowest,
}

impl World {
    fn parse_input(input: &str) -> Self {
        fn find_special_position(grid: &Vec<Vec<char>>, ch: char) -> (usize, usize) {
            grid.iter()
                .enumerate()
                .find_map(|row| {
                    row.1
                        .iter()
                        .enumerate()
                        .find(|val| *val.1 == ch)
                        .map(|val| (row.0, val.0))
                })
                .unwrap()
        }

        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let start = find_special_position(&grid, 'S');
        let end = find_special_position(&grid, 'E');
        let heights = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|ch| match ch {
                        'S' => 0,
                        'E' => 25,
                        _ => (ch as u32 - 'a' as u32) as i32,
                    })
                    .collect()
            })
            .collect();

        Self {
            start,
            end,
            heights,
        }
    }

    fn find_shortest(&self, source_to_consider: &SourceToConsider) -> i32 {
        let mut dist = HashMap::new();
        let mut visited = HashSet::new();
        let mut next_to_process = BinaryHeap::new();

        #[derive(Debug, Eq, PartialEq)]
        struct NextToProcess {
            dist: i32,
            coord: (usize, usize),
        }

        impl Ord for NextToProcess {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                if self.dist != other.dist {
                    self.dist.cmp(&other.dist).reverse()
                } else {
                    self.coord.cmp(&other.coord)
                }
            }
        }

        impl PartialOrd for NextToProcess {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        fn neighbour(world: &World, coord: &(usize, usize)) -> Vec<(usize, usize)> {
            let mut results = vec![];

            if coord.0 != 0 {
                results.push((coord.0 - 1, coord.1));
            }

            if coord.0 + 1 < world.heights.len() {
                results.push((coord.0 + 1, coord.1));
            }

            if coord.1 != 0 {
                results.push((coord.0, coord.1 - 1));
            }

            if coord.1 + 1 < world.heights[0].len() {
                results.push((coord.0, coord.1 + 1));
            }

            results
        }

        match source_to_consider {
            SourceToConsider::OnlyStart => {
                dist.insert(self.start, 0);
                next_to_process.push(NextToProcess {
                    dist: 0,
                    coord: self.start,
                });
            }
            SourceToConsider::AllLowest => {
                self.heights
                    .iter()
                    .enumerate()
                    .flat_map(|(row_index, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, cell)| **cell == 0)
                            .map(|(col_index, _)| (row_index, col_index))
                            .collect::<Vec<_>>()
                    })
                    .for_each(|coord| {
                        dist.insert(coord, 0);
                        next_to_process.push(NextToProcess { dist: 0, coord });
                    });
            }
        }

        while let Some(current) = next_to_process.pop() {
            if visited.contains(&current.coord) {
                continue;
            }
            if current.coord == self.end {
                return *dist.get(&current.coord).unwrap();
            }

            visited.insert(current.coord);
            neighbour(self, &current.coord)
                .into_iter()
                .filter(|coord| !visited.contains(&coord))
                .filter(|coord| {
                    self.heights[coord.0][coord.1]
                        - self.heights[current.coord.0][current.coord.1].abs()
                        <= 1
                })
                .map(|coord| (coord, current.dist + 1))
                .filter(|(coord, newdist)| dist.get(&coord).unwrap_or(&i32::MAX) > newdist)
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(coord, newdist)| {
                    dist.insert(coord, newdist);
                    next_to_process.push(NextToProcess {
                        dist: newdist,
                        coord,
                    });
                });
        }

        unreachable!("Input must always have an answer.");
    }
}

fn p1(input: &str) -> String {
    World::parse_input(input)
        .find_shortest(&SourceToConsider::OnlyStart)
        .to_string()
}

fn p2(input: &str) -> String {
    World::parse_input(input)
        .find_shortest(&SourceToConsider::AllLowest)
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
        assert_eq!(p1(SAMPLE_INPUT), "31");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "350");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "29");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "349");
    }
}
