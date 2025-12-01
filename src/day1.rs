#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.replace("L", "-").replace("R", "").parse::<i64>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> u32 {
    let mut state = 50;
    let mut count = 0;
    for i in input {
        state = ((state + i) % 100 + 100) % 100;
        if state == 0 {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut state = 50;
    let mut count = 0;
    for i in input {
        if *i >= 0 {
            state += i;
            count += state / 100;
            state %= 100;
        } else {
            if state == 0 {
                count += i / -100;
            } else if -i >= state {
                count += ((state + i) / -100) + 1;
            }
            state = ((state + i) % 100 + 100) % 100;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works_r1000() {
        let result = solve_part2(&[1000]);
        assert_eq!(result, 10);
    }

    #[test]
    fn part2_works_l1000() {
        let result = solve_part2(&[-1000]);
        assert_eq!(result, 10);
    }

    #[test]
    fn part2_works_r50() {
        let result = solve_part2(&[50]);
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_works_l50() {
        let result = solve_part2(&[-50]);
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_works_test1() {
        let result = solve_part2(&[-50, -100]);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works_test2() {
        let result = solve_part2(&[-50, 50]);
        assert_eq!(result, 1);
    }
}
