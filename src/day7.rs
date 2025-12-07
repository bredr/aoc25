use std::collections::{HashMap, HashSet};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let mut splitters = HashSet::<(i32, i32)>::new();
    let mut start: (i32, i32) = (0, 0);
    input.lines().enumerate().for_each(|(idy, row)| {
        row.split("").into_iter().enumerate().for_each(|(idx, v)| {
            if v == "^" {
                splitters.insert((idx as i32, idy as i32));
            }
            if v == "S" {
                start = (idx as i32, idy as i32);
            }
        });
    });
    let max_depth = input.lines().count();
    Input {
        splitters,
        start,
        max_depth,
    }
}

pub struct Input {
    pub splitters: HashSet<(i32, i32)>,
    pub start: (i32, i32),
    pub max_depth: usize,
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut splits = 0;
    let mut beam_tips = HashSet::<(i32, i32)>::new();
    beam_tips.insert(input.start.clone());
    (1..input.max_depth).for_each(|depth| {
        beam_tips = HashSet::from_iter(beam_tips.iter().flat_map(|(x, _)| {
            let next_pos = (*x, depth as i32);
            if input.splitters.contains(&next_pos) {
                splits += 1;
                vec![(x - 1, depth as i32), (x + 1, depth as i32)]
            } else {
                vec![next_pos]
            }
        }))
    });
    splits
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut beam_tips = HashMap::<(i32, i32), usize>::new();
    beam_tips.insert(input.start.clone(), 1);
    (1..input.max_depth).for_each(|depth| {
        let mut next_beam_tips = HashMap::<(i32, i32), usize>::new();
        beam_tips
            .iter()
            .flat_map(|(k, count)| {
                let next_pos = (k.0, depth as i32);
                if input.splitters.contains(&next_pos) {
                    vec![
                        ((k.0 - 1, depth as i32), count),
                        ((k.0 + 1, depth as i32), count),
                    ]
                } else {
                    vec![(next_pos, count)]
                }
            })
            .for_each(|(k, v)| {
                if next_beam_tips.contains_key(&k) {
                    *next_beam_tips.get_mut(&k).unwrap() += v;
                } else {
                    next_beam_tips.insert(k, v.clone());
                }
            });
        beam_tips = next_beam_tips;
    });
    beam_tips.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = input_generator(".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............");
        let result = solve_part1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_example() {
        let input = input_generator(".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............");
        let result = solve_part2(&input);
        assert_eq!(result, 40);
    }
}
