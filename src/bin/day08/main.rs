const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| (ch as u32 - '0' as u32) as i32)
                .collect()
        })
        .collect()
}

fn p1(input: &str) -> String {
    let grid = parse_input(input);

    let mut visible = std::iter::repeat(
        std::iter::repeat(false)
            .take(grid[0].len())
            .collect::<Vec<_>>(),
    )
    .take(grid.len())
    .collect::<Vec<_>>();

    grid.iter()
        .zip(visible.iter_mut())
        .for_each(|(grid_row, visible_row)| {
            let mut edge = -1;
            grid_row
                .iter()
                .zip(visible_row.iter_mut())
                .for_each(|(grid, visible)| {
                    if *grid > edge {
                        edge = *grid;
                        *visible = true;
                    }
                });

            edge = -1;

            grid_row
                .iter()
                .zip(visible_row.iter_mut())
                .rev()
                .for_each(|(grid, visible)| {
                    if *grid > edge {
                        edge = *grid;
                        *visible = true;
                    }
                });
        });

    (0..grid[0].len()).into_iter().for_each(|col| {
        let mut edge = -1;

        grid.iter()
            .zip(visible.iter_mut())
            .for_each(|(grid_row, visible_row)| {
                let grid = grid_row.iter().nth(col).unwrap();
                let visible = visible_row.iter_mut().nth(col).unwrap();
                if *grid > edge {
                    edge = *grid;
                    *visible = true;
                }
            });

        edge = -1;

        grid.iter()
            .zip(visible.iter_mut())
            .rev()
            .for_each(|(grid_row, visible_row)| {
                let grid = grid_row.iter().nth(col).unwrap();
                let visible = visible_row.iter_mut().nth(col).unwrap();
                if *grid > edge {
                    edge = *grid;
                    *visible = true;
                }
            });
    });

    visible
        .into_iter()
        .map(|row| row.into_iter().filter(|v| *v).count())
        .sum::<usize>()
        .to_string()
}

struct TraverseIter {
    pos: (i32, i32),
    dir: (i32, i32),
    max: (i32, i32),
    ended: bool,
}

impl TraverseIter {
    fn traverse(start: &(usize, usize), dir: &(i32, i32), max: &(usize, usize)) -> Self {
        Self {
            pos: (start.0 as i32, start.1 as i32),
            dir: *dir,
            max: (max.0 as i32, max.1 as i32),
            ended: false,
        }
    }
}

impl Iterator for TraverseIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.ended {
            true => None,
            false => {
                let result = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
                self.pos = result;

                if result.0 < 0 || result.0 >= self.max.0 || result.1 < 0 || result.1 >= self.max.1
                {
                    self.ended = true;
                    None
                } else {
                    Some((result.0 as usize, result.1 as usize))
                }
            }
        }
    }
}

fn p2(input: &str) -> String {
    let grid = parse_input(input);

    let max = (grid.len(), grid[0].len());
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    (0..grid.len())
        .into_iter()
        .map(|r| {
            (0..grid[0].len())
                .into_iter()
                .map(|c| {
                    let pos = (r, c);

                    directions
                        .iter()
                        .map(|dir| {
                            TraverseIter::traverse(&pos, &dir, &max)
                                .position(|pos| grid[pos.0][pos.1] >= grid[r][c])
                                .map(|v| v + 1)
                                .unwrap_or_else(|| TraverseIter::traverse(&pos, &dir, &max).count())
                                as u32
                        })
                        .product::<u32>()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
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
        assert_eq!(p1(SAMPLE_INPUT), "21");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1796");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "8");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "288120");
    }
}
