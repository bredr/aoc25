use itertools::Itertools;
use regress::Regex;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|s| {
            s.split("-")
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(i64, i64)]) -> i64 {
    let mut result: i64 = 0;
    let repeats = Regex::new(r"^(\d+)\1$").unwrap();
    for range in input.iter() {
        for i in range.0..=range.1 {
            let istr = i.to_string();
            if repeats.find(istr.as_str()).is_some() {
                result += i;
            }
        }
    }
    result
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(i64, i64)]) -> i64 {
    let mut result: i64 = 0;
    let has_repeats_first = Regex::new(r"^(\d+)\1").unwrap();
    for range in input.iter() {
        for i in range.0..=range.1 {
            let istr = i.to_string();
            if has_repeats_first.find(istr.as_str()).is_some() {
                for factor in list_factors(istr.len()) {
                    let sub_str = &istr[0..factor];
                    if sub_str.repeat(istr.len() / factor) == istr {
                        result += i;
                        break;
                    }
                }
            }
        }
    }
    result
}

fn list_factors(x: usize) -> Vec<usize> {
    (1..=x / 2).into_iter().filter(|i| x % i == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = solve_part1(&[(11, 22)]);
        assert_eq!(result, 33);
    }

    #[test]
    fn part1_test2() {
        let result = solve_part1(&[(998, 1012)]);
        assert_eq!(result, 1010);
    }

    #[test]
    fn part2_test1() {
        let result = solve_part2(&[(998, 1012)]);
        assert_eq!(result, 1010 + 999);
    }

    #[test]
    fn part2_test2() {
        let result = solve_part2(&[(222220, 222224)]);
        assert_eq!(result, 222222);
    }

    #[test]
    fn part2_test3() {
        let result = solve_part2(&[(1188511880, 1188511890)]);
        assert_eq!(result, 1188511885);
    }

    #[test]
    fn part2_test4() {
        let result = solve_part2(&[(565653, 565659)]);
        assert_eq!(result, 565656);
    }

    #[test]
    fn part2_test5() {
        let result = solve_part2(&[(2121212118, 2121212124)]);
        assert_eq!(result, 2121212121);
    }

    #[test]
    fn part2_test6() {
        let result = solve_part2(&[(824824821, 824824827)]);
        assert_eq!(result, 824824824);
    }

    #[test]
    fn part2_test7() {
        let result = solve_part2(&[(123123123123123123, 123123123123123124)]);
        assert_eq!(result, 123123123123123123);
    }

    #[test]
    fn part2_test8() {
        let result = solve_part2(&[(11111111111111111, 11111111111111112)]);
        assert_eq!(result, 11111111111111111);
    }

    #[test]
    fn part2_test9() {
        let result = solve_part2(&[(11, 22)]);
        assert_eq!(result, 33);
    }

    #[test]
    fn part2_test10() {
        let result = solve_part2(&[(1111222211112222, 1111222211112223)]);
        assert_eq!(result, 1111222211112222);
    }

    #[test]
    fn part2_example() {
        let input = input_generator(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 4174379265);
    }
}
