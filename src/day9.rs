use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[(i64, i64)]) -> i64 {
    (0..input.len())
        .combinations(2)
        .filter(|x| x[0] != x[1])
        .map(|x| area(input[x[0]], input[x[1]]))
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[(i64, i64)]) -> i64 {
    let boundary = Polygon::new(input.to_vec());
    (0..input.len())
        .combinations(2)
        .map(|x| (x.clone(), area(input[x[0]], input[x[1]])))
        .sorted_by_key(|x| -x.1)
        .filter(|(x, _)| boundary.area_valid(x[0], x[1]))
        .take(1)
        .next()
        .unwrap()
        .1
}

fn area(a: (i64, i64), b: (i64, i64)) -> i64 {
    ((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)) as i64
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone)]
pub struct Polygon {
    vertices: Vec<Point>,
    filled: HashSet<Point>,
}

impl Polygon {
    fn new(vertices: Vec<(i64, i64)>) -> Self {
        let compressed_x = vertices
            .iter()
            .map(|(x, _)| x.clone())
            .unique()
            .sorted()
            .enumerate()
            .collect_vec();
        let x_to_compressed_x = HashMap::<i64, i64>::from_iter(
            compressed_x
                .iter()
                .map(|(a, b)| (b.clone(), a.clone() as i64)),
        );
        let compressed_y = vertices
            .iter()
            .map(|(_, y)| y.clone())
            .sorted()
            .unique()
            .enumerate()
            .collect_vec();
        let y_to_compressed_y = HashMap::<i64, i64>::from_iter(
            compressed_y
                .iter()
                .map(|(a, b)| (b.clone(), a.clone() as i64)),
        );

        let compressed_vertices = vertices
            .iter()
            .map(|x| Point {
                x: x_to_compressed_x[&x.0],
                y: y_to_compressed_y[&x.1],
            })
            .collect_vec();

        let mut inner_tiles = HashSet::<Point>::new();
        let mut boundary_tiles = HashSet::<Point>::from_iter(compressed_vertices.clone());

        compressed_vertices
            .iter()
            .circular_tuple_windows()
            .for_each(|(a, b)| {
                if a.x == b.x {
                    boundary_tiles.extend(
                        (a.y.min(b.y) + 1..a.y.max(b.y)).map(|y| Point { x: a.x.clone(), y }),
                    );
                } else {
                    boundary_tiles.extend(
                        (a.x.min(b.x) + 1..a.x.max(b.x)).map(|x| Point { x, y: a.y.clone() }),
                    );
                }
            });

        let mut inside = false;
        let mut seed: Option<Point> = None;
        let mut x: i64 = -1;
        let mut y: i64 = 1;
        while seed == None {
            let point = Point {
                x: x as i64,
                y: (compressed_y.len() / 3) as i64,
            };
            if boundary_tiles.contains(&point) {
                inside = !inside;
            }
            if inside && !boundary_tiles.contains(&point) {
                seed = Some(point);
                break;
            }
            x += 1;
            if x == compressed_x.len() as i64 {
                y += 1;
                x = -1;
            }
            if y == compressed_y.len() as i64 {
                seed = Some(Point { x: 0, y: 0 });
                break;
            }
        }

        boundary_fill(&mut inner_tiles, &boundary_tiles, seed.unwrap());

        inner_tiles.extend(boundary_tiles);

        Polygon {
            vertices: compressed_vertices,
            filled: inner_tiles,
        }
    }

    fn area_valid(&self, a: usize, b: usize) -> bool {
        let a_pt = self.vertices[a].clone();
        let b_pt = self.vertices[b].clone();
        (a_pt.x.min(b_pt.x)..=a_pt.x.max(b_pt.x))
            .flat_map(move |x| {
                (a_pt.y.min(b_pt.y)..=a_pt.y.max(b_pt.y)).map(move |y| Point { x, y })
            })
            .all(|x| self.filled.contains(&x))
    }
}

fn boundary_fill(inner_tiles: &mut HashSet<Point>, bound_tiles: &HashSet<Point>, tile: Point) {
    if !inner_tiles.contains(&tile) && !bound_tiles.contains(&tile) {
        inner_tiles.insert(tile);
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x,
                y: tile.y + 1,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x - 1,
                y: tile.y,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x + 1,
                y: tile.y,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x,
                y: tile.y - 1,
            },
        );

        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x + 1,
                y: tile.y + 1,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x - 1,
                y: tile.y + 1,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x + 1,
                y: tile.y - 1,
            },
        );
        boundary_fill(
            inner_tiles,
            bound_tiles,
            Point {
                x: tile.x - 1,
                y: tile.y - 1,
            },
        );
    }
}
