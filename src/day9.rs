use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[(i64, i64)]) -> i64 {
    (0..input.len())
        .combinations(2)
        .filter(|x| x[0] != x[1])
        .map(|x| area(input[x[0]], input[x[1]]))
        .max()
        .unwrap()
}

fn area(a: (i64, i64), b: (i64, i64)) -> i64 {
    ((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)) as i64
}
