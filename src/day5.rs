use std::ops::RangeInclusive;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let [ranges, ids] = input.split("\n\n").take(2).collect::<Vec<&str>>()[..2] else {
        todo!()
    };
    Input {
        ranges: ranges
            .trim()
            .lines()
            .map(|r| {
                let [lower, upper] = r
                    .trim()
                    .split("-")
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()[..2]
                else {
                    todo!()
                };
                lower..=upper
            })
            .collect::<Vec<RangeInclusive<i64>>>(),
        ids: ids
            .trim()
            .lines()
            .map(|id| id.parse::<i64>().unwrap())
            .collect(),
    }
}

pub struct Input {
    pub ranges: Vec<RangeInclusive<i64>>,
    pub ids: Vec<i64>,
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .ids
        .iter()
        .filter(|x| input.ranges.iter().any(|range| range.contains(*x)))
        .count()
}
