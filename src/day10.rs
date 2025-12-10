use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::usize;
use z3::ast;
use z3::{ast::Int, Optimize};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    let match_indicator_lights = Regex::new(r"\[(.*?)\]").unwrap();
    let match_wiring_schematics = Regex::new(r"\((.*?)\)").unwrap();
    let joltage_requirements = Regex::new(r"\{(.*?)\}").unwrap();

    input
        .lines()
        .map(|l| {
            let n_lights = match_indicator_lights
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .len();
            Machine {
                indicator_lights: match_indicator_lights
                    .captures(l)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect(),
                wiring_schematics: match_wiring_schematics
                    .captures_iter(l)
                    .map(|m| {
                        let match_str = m.get(1).unwrap().as_str();
                        let idxs = match_str
                            .split(",")
                            .map(|d| d.parse::<usize>().unwrap())
                            .collect_vec();
                        let mut schematic = vec![false; n_lights];
                        idxs.iter().for_each(|idx| {
                            schematic[*idx] = true;
                        });
                        schematic
                    })
                    .collect_vec(),
                joltage_requirements: joltage_requirements
                    .captures(l)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec()
}

#[derive(Debug)]
pub struct Machine {
    indicator_lights: Vec<bool>,
    wiring_schematics: Vec<Vec<bool>>,
    joltage_requirements: Vec<usize>,
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Machine]) -> usize {
    input.iter().map(|x| x.search_p1()).sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Machine]) -> i64 {
    input.iter().map(|x| x.solve_p2()).sum()
}

impl Machine {
    fn init_state_p1(&self) -> Vec<bool> {
        vec![false; self.indicator_lights.len()]
    }

    fn search_p1(&self) -> usize {
        let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
        let mut visited: HashSet<Vec<bool>> = HashSet::new();

        let initial_state = self.init_state_p1();
        let target_state = &self.indicator_lights;

        queue.push_back((initial_state.clone(), 0));
        visited.insert(initial_state);

        while let Some((current_state, depth)) = queue.pop_front() {
            if &current_state == target_state {
                return depth;
            }
            for button in self.wiring_schematics.iter() {
                let next_state = toggle(&current_state, button);
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, depth + 1));
                }
            }
        }
        panic!("Target state is unreachable!");
    }

    fn solve_p2(&self) -> i64 {
        let solver = Optimize::new();

        let buttons: Vec<Int> = (0..self.wiring_schematics.len())
            .map(|idx| Int::new_const(idx as u32))
            .collect_vec();
        buttons.iter().for_each(|button| {
            solver.assert(&button.ge(0));
        });
        self.joltage_requirements
            .iter()
            .enumerate()
            .for_each(|(idx, target)| {
                solver.assert(
                    &buttons
                        .iter()
                        .zip(self.wiring_schematics.clone())
                        .filter(|(_, schematic)| schematic[idx])
                        .map(|(button, _)| button)
                        .fold(Int::from_i64(0), |acc, button_ref| acc + button_ref)
                        .eq(ast::Int::from_i64(*target as i64)),
                );
            });

        let pushes = buttons.into_iter().reduce(|a, b| a + b).unwrap();

        solver.minimize(&pushes);
        if let z3::SatResult::Sat = solver.check(&[]) {
            let model = solver.get_model().unwrap();
            return model.eval(&pushes, true).unwrap().as_i64().unwrap();
        }
        panic!("no solution")
    }
}

fn toggle(indicator_lights: &Vec<bool>, button: &Vec<bool>) -> Vec<bool> {
    indicator_lights
        .iter()
        .zip(button)
        .map(|(a, b)| a ^ b)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = input_generator(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 40);
    }

    #[test]
    fn part2_example() {
        let input = input_generator(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 33);
    }
}
