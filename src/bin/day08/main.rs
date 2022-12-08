const ACTUAL_INPUT: &str = include_str!("./input.txt");

fn p1(input: &str) -> String {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| (ch as u32 - '0' as u32) as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "21");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1796");
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
