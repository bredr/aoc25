use itertools::Itertools;
use regex::Regex;

fn transpose(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let cols = input[0].len();
    let rows = input.len();
    (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let rows: Vec<Vec<i64>> = input
        .lines()
        .take_while(|line| !line.contains("+"))
        .map(|line| {
            line.split(char::is_whitespace)
                .filter(|x| x.trim().len() > 0)
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let cols = transpose(rows);
    let ops = input.lines().tail(1).collect::<Vec<&str>>()[0]
        .split(char::is_whitespace)
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.trim().chars().into_iter().next().unwrap())
        .collect::<Vec<char>>();

    cols.iter()
        .enumerate()
        .map(|(idx, col)| {
            let op = ops[idx];
            col.iter()
                .copied()
                .reduce(|acc, x| match op {
                    '+' => acc + x,
                    '*' => acc * x,
                    _ => acc,
                })
                .unwrap()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let ops_line = input.lines().last().unwrap();
    let re = Regex::new(r"[\+\*\-\\]\s+").unwrap();
    re.captures_iter(ops_line)
        .map(|c| {
            let m = c.get_match();
            let op = &ops_line[m.start()..m.end()]
                .trim()
                .chars()
                .into_iter()
                .next()
                .unwrap();
            (m.start()..m.end())
                .map(|idx| {
                    input
                        .lines()
                        .take_while(|x| !x.contains("+"))
                        .map(|x| x[idx..idx + 1].trim())
                        .filter(|x| x.len() > 0)
                        .join("")
                        .parse::<i64>()
                })
                .filter(Result::is_ok)
                .map(|x| x.unwrap())
                .reduce(|acc, n| match op {
                    '+' => acc + n,
                    '*' => acc * n,
                    _ => acc,
                })
                .into_iter()
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result =
            solve_part1("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part2_example() {
        let result =
            solve_part2("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(result, 3263827);
    }
}
