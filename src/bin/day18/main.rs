use std::collections::{HashMap, HashSet, VecDeque};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2022/18/input.txt");

type Surface = (i32, i32, i32, i32, i32, i32);
type Cube = (i32, i32, i32);

fn sides(cube: &Cube) -> Vec<Surface> {
    vec![
        (cube.0, cube.0, cube.1, cube.1 + 1, cube.2, cube.2 + 1),
        (
            cube.0 + 1,
            cube.0 + 1,
            cube.1,
            cube.1 + 1,
            cube.2,
            cube.2 + 1,
        ),
        (cube.0, cube.0 + 1, cube.1, cube.1, cube.2, cube.2 + 1),
        (
            cube.0,
            cube.0 + 1,
            cube.1 + 1,
            cube.1 + 1,
            cube.2,
            cube.2 + 1,
        ),
        (cube.0, cube.0 + 1, cube.1, cube.1 + 1, cube.2, cube.2),
        (
            cube.0,
            cube.0 + 1,
            cube.1,
            cube.1 + 1,
            cube.2 + 1,
            cube.2 + 1,
        ),
    ]
}

fn get_droplets(input: &str) -> HashSet<Cube> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|value| value.parse::<i32>().unwrap());
            (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )
        })
        .collect()
}

fn get_p1_surfaces(droplets: &HashSet<Cube>) -> HashSet<Surface> {
    let mut seen = HashMap::new();

    droplets.iter().for_each(|cube| {
        sides(cube).into_iter().for_each(|surface| {
            *seen.entry(surface).or_insert(0) += 1;
        });
    });

    seen.into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(surface, _)| surface)
        .collect()
}

fn get_p2_surfaces(droplets: &HashSet<Cube>, p1_surfaces: &HashSet<Surface>) -> HashSet<Surface> {
    let mut outside = HashSet::from([(0, 0, 0)]);
    let mut queue = VecDeque::from([(0, 0, 0)]);

    while let Some(current) = queue.pop_front() {
        let neighbours = [
            (current.0 - 1, current.1, current.2),
            (current.0 + 1, current.1, current.2),
            (current.0, current.1 - 1, current.2),
            (current.0, current.1 + 1, current.2),
            (current.0, current.1, current.2 - 1),
            (current.0, current.1, current.2 + 1),
        ]
        .into_iter()
        .filter(|cube| {
            cube.0 >= -1
                && cube.1 >= -1
                && cube.2 >= -1
                && cube.0 <= 22
                && cube.1 <= 22
                && cube.2 <= 22
                && !droplets.contains(cube)
                && !outside.contains(cube)
        })
        .collect::<Vec<_>>();
        outside.extend(neighbours.clone());
        queue.extend(neighbours);
    }

    p1_surfaces
        .difference(
            &p1_surfaces
                .iter()
                .filter(|surface| surface.0 == surface.1)
                .map(|surface| (surface.0, surface.2, surface.4))
                .filter(|cube| !droplets.contains(cube) && !outside.contains(cube))
                .map(|cube| sides(&cube))
                .flatten()
                .collect(),
        )
        .copied()
        .collect()
}

fn p1(input: &str) -> String {
    get_p1_surfaces(&get_droplets(input)).len().to_string()
}

fn p2(input: &str) -> String {
    let droplets = get_droplets(input);
    get_p2_surfaces(&droplets, &get_p1_surfaces(&droplets))
        .len()
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
        assert_eq!(p1(SAMPLE_INPUT), "64");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4244");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "58");
    }

    #[test]
    fn test_p2_actual() {
        // 2528 is too high
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
