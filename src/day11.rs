use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<String>> {
    HashMap::<String, Vec<String>>::from_iter(input.lines().map(|l| {
        let (input, out) = l.split(": ").collect_tuple().expect("invalid format");
        (
            input.trim().to_owned(),
            out.split(" ").map(|x| x.trim().to_owned()).collect(),
        )
    }))
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &HashMap<String, Vec<String>>) -> usize {
    let mut network = Network::new(input);
    network.paths(String::from("you"), String::from("out"), HashSet::new())
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &HashMap<String, Vec<String>>) -> usize {
    let mut network = Network::new(input);

    network.paths(String::from("svr"), String::from("dac"), HashSet::new())
        * network.paths(String::from("dac"), String::from("fft"), HashSet::new())
        * network.paths(String::from("fft"), String::from("out"), HashSet::new())
        + network.paths(String::from("svr"), String::from("fft"), HashSet::new())
            * network.paths(String::from("fft"), String::from("dac"), HashSet::new())
            * network.paths(String::from("dac"), String::from("out"), HashSet::new())
}

struct Network {
    edges: HashMap<String, Vec<String>>,
    cache: HashMap<(String, String), usize>,
}

impl Network {
    fn new(network: &HashMap<String, Vec<String>>) -> Self {
        Self {
            edges: network.clone(),
            cache: HashMap::new(),
        }
    }

    fn paths(&mut self, current: String, target: String, visited: HashSet<String>) -> usize {
        match self.cache.get(&(current.clone(), target.clone())) {
            Some(hit) => hit.clone(),
            None => {
                let mut next_visited = visited.clone();
                next_visited.insert(current.to_owned());
                if current == target {
                    self.cache.insert((current.clone(), target.clone()), 1);
                    return 1;
                } else {
                    match self.edges.clone().get(&current) {
                        Some(options) => {
                            let result = options
                                .iter()
                                .cloned()
                                .filter(|next| !visited.contains(next))
                                .map(|next| {
                                    self.paths(
                                        next.to_owned(),
                                        target.clone(),
                                        next_visited.clone(),
                                    )
                                })
                                .sum();
                            self.cache.insert((current.clone(), target.clone()), result);
                            return result;
                        }
                        None => {
                            self.cache.insert((current.clone(), target.clone()), 0);
                            return 0;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out";

    #[test]
    fn part2_example() {
        let input = input_generator(INPUT_STR);
        let result = solve_part2(&input);
        assert_eq!(result, 2);
    }
}
