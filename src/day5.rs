use std::cmp;
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

trait RangeExtended<T> {
    /// Checks if ranges overlap. If they do, returns the Union (merge) of both.
    fn merge_if_overlapping(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
}

impl<T> RangeExtended<T> for RangeInclusive<T>
where
    T: Ord + Clone,
{
    fn merge_if_overlapping(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>> {
        if self.start() <= other.end() && other.start() <= self.end() {
            let new_start = cmp::min(self.start(), other.start());
            let new_end = cmp::max(self.end(), other.end());

            Some(new_start.clone()..=new_end.clone())
        } else {
            None
        }
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut ranges = input.ranges.clone();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut exclusive_ranges: Vec<RangeInclusive<i64>> = vec![ranges.get(0).unwrap().clone()];

    ranges[1..].iter().for_each(|range| {
        match exclusive_ranges
            .clone()
            .iter()
            .enumerate()
            .find(|(idx, r)| match r.merge_if_overlapping(range) {
                Some(update) => {
                    exclusive_ranges[*idx] = update.clone();
                    true
                }
                None => false,
            }) {
            Some(_) => {}
            None => {
                exclusive_ranges.push(range.clone());
            }
        }
    });
    exclusive_ranges.iter().map(|r| r.clone().count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_if_overlapping_test1() {
        assert_eq!((1..=4).merge_if_overlapping(&(0..=2)), Some(0..=4))
    }

    #[test]
    fn merge_if_overlapping_test2() {
        assert_eq!((1..=4).merge_if_overlapping(&(2..=5)), Some(1..=5))
    }

    #[test]
    fn merge_if_overlapping_test3() {
        assert_eq!((1..=3).merge_if_overlapping(&(4..=5)), None)
    }

    #[test]
    fn merge_if_overlapping_test4() {
        assert_eq!((10..=18).merge_if_overlapping(&(16..=20)), Some(10..=20))
    }

    #[test]
    fn solve_part2_example() {
        let input = input_generator("3-5\n10-14\n16-20\n12-18\n\n1");
        let result = solve_part2(&input);
        assert_eq!(result, 14);
    }
}
