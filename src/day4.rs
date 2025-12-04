use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(idx, l)| {
            l.trim()
                .split("")
                .enumerate()
                .flat_map(|(idy, x)| {
                    if x == "@" {
                        vec![(idx as i32, idy as i32)]
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<HashSet<(i32, i32)>>()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &HashSet<(i32, i32)>) -> i64 {
    input
        .iter()
        .map(|(idx, idy)| {
            if vec![
                input.contains(&(idx + 1, idy + 0)),
                input.contains(&(idx - 1, idy + 0)),
                input.contains(&(idx + 1, idy - 1)),
                input.contains(&(idx + 0, idy - 1)),
                input.contains(&(idx - 1, idy - 1)),
                input.contains(&(idx + 1, idy + 1)),
                input.contains(&(idx + 0, idy + 1)),
                input.contains(&(idx - 1, idy + 1)),
            ]
            .iter()
            .map(|x| if *x { 1 } else { 0 })
            .sum::<i32>()
                < 4
            {
                1
            } else {
                0
            }
        })
        .sum()
}
