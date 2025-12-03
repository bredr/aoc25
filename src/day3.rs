use std::cmp::Reverse;

use itertools::Itertools;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split("")
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> i64 {
    input
        .into_iter()
        .map(|r| {
            let mut rr = r.to_vec();
            rr.sort_by(|a, b| b.cmp(a));
            let mut result = 0;
            for max in rr.iter() {
                let mut has_update = false;
                for (idx, v) in r.iter().enumerate() {
                    if *v == *max && idx + 1 < r.len() {
                        let max2 = r[idx + 1..].iter().max().unwrap().clone();
                        let pos_result = vec![*max, max2].iter().join("").parse::<i64>().unwrap();
                        if pos_result > result {
                            result = pos_result;
                            has_update = true;
                        }
                    }
                }
                if has_update {
                    break;
                }
            }
            result
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> i128 {
    input
        .iter()
        .map(|r| {
            let mut idx = 0;
            let mut joltage = 0;
            for end in (1..=12).rev() {
                let x = r[idx..=(r.len() - end)]
                    .iter()
                    .map(|x| *x as i128)
                    .enumerate()
                    .min_by_key(|&(_, value)| Reverse(value))
                    .unwrap()
                    .clone();
                idx += x.0 + 1;
                joltage += x.1 * (10 as u64).pow(end as u32 - 1) as i128;
            }
            joltage
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input =
            input_generator("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        let result = solve_part1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn part2_example() {
        let input =
            input_generator("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        let result = solve_part2(&input);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn part2_test1() {
        let input = input_generator("818181911112111");
        let result = solve_part2(&input);
        assert_eq!(result, 888911112111);
    }
}
