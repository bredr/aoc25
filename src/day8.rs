use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    Input {
        boxes: input
            .lines()
            .map(|l| {
                l.trim()
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<Vec<_>>(),
        connections: 1000,
    }
}

pub struct Input {
    pub boxes: Vec<(i64, i64, i64)>,
    pub connections: usize,
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let n = input.boxes.len();

    let mut joined_boxes = HashSet::<(usize, usize)>::from_iter(
        (0..n)
            .combinations(2)
            .filter(|x| x[0] != x[1])
            .map(|x| {
                (
                    x[0],
                    x[1],
                    straight_line_distance(input.boxes[x[0]], input.boxes[x[1]]),
                )
            })
            .sorted_by_key(|a| a.2)
            .take(input.connections)
            .map(|(a, b, _)| (a, b)),
    );
    let mut circuits: Vec<HashSet<usize>> = vec![];
    while joined_boxes.len() > 0 {
        let Some(next) = joined_boxes.iter().next() else {
            break;
        };
        let mut current_circuit = HashSet::<usize>::new();
        current_circuit.insert(next.0);
        current_circuit.insert(next.1);
        let mut last_size = current_circuit.len();
        loop {
            for (a, b) in joined_boxes.clone().iter() {
                if current_circuit.contains(a) || current_circuit.contains(b) {
                    joined_boxes.remove(&(a.clone(), b.clone()));
                    current_circuit.insert(a.clone());
                    current_circuit.insert(b.clone());
                }
            }
            if current_circuit.len() == last_size {
                break;
            }
            last_size = current_circuit.len();
        }
        circuits.push(current_circuit);
    }
    circuits
        .iter()
        .map(|x| x.len() as i64)
        .sorted_by_key(|x| -x)
        .take(3)
        .product::<i64>()
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<u64>,
    n: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
            n,
        }
    }
    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let s1 = self.find(x);
        let s2 = self.find(y);
        if s1 != s2 {
            if self.rank[s1] < self.rank[s2] {
                self.parent[s1] = s2;
            } else if self.rank[s1] > self.rank[s2] {
                self.parent[s2] = s1;
            } else {
                self.parent[s2] = s1;
                self.rank[s1] += 1;
            }
            self.n -= 1;
        }
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let n = input.boxes.len();
    let joined_boxes = (0..n)
        .combinations(2)
        .filter(|x| x[0] != x[1])
        .map(|x| {
            (
                x[0],
                x[1],
                straight_line_distance(input.boxes[x[0]], input.boxes[x[1]]),
            )
        })
        .sorted_by_key(|a| a.2);
    let mut dsu = DSU::new(n);
    for (a, b, _) in joined_boxes {
        dsu.union(a, b);
        if dsu.n == 1 {
            return input.boxes[a].0 * input.boxes[b].0;
        }
    }
    0
}

fn straight_line_distance(x: (i64, i64, i64), y: (i64, i64, i64)) -> i64 {
    let (x_1, x_2, x_3) = x;
    let (y_1, y_2, y_3) = y;
    (x_1 - y_1).pow(2) + (x_2 - y_2).pow(2) + (x_3 - y_3).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let mut input = input_generator(
            "162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689",
        );
        input.connections = 10;
        let result = solve_part1(&input);
        assert_eq!(result, 40);
    }

    #[test]
    fn part2_example() {
        let input = input_generator(
            "162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 25272);
    }
}
