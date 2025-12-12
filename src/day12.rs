use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let grid_match = Regex::new(r"\d*x\d*\:\s[\s\d]*\n").unwrap();
    let grids = grid_match
        .find_iter(input)
        .map(|m| {
            let (shape_part, counts) = m.as_str().split(":").collect_tuple().expect("wrong shape");
            Grid {
                shape: shape_part
                    .split("x")
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect_tuple()
                    .expect("wrong shape"),
                to_fit: counts
                    .trim()
                    .split(" ")
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec();
    let shape_match = Regex::new(r"[\.\#]+\n[\.\#]+\n[\.\#]+").unwrap();
    let shapes = shape_match
        .find_iter(input)
        .map(|m| Shape {
            area: m.as_str().trim().chars().filter(|x| *x == '#').count(),
        })
        .collect_vec();
    Input { shapes, grids }
}

pub struct Input {
    shapes: Vec<Shape>,
    grids: Vec<Grid>,
}

struct Shape {
    area: usize,
}

struct Grid {
    shape: (usize, usize),
    to_fit: Vec<usize>,
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .grids
        .iter()
        .filter(|grid| fits_shapes(&grid, &input.shapes))
        .count()
}

fn fits_shapes(grid: &Grid, shapes: &Vec<Shape>) -> bool {
    (grid.shape.0 * grid.shape.1)
        >= grid
            .to_fit
            .iter()
            .enumerate()
            .map(|(idx, count)| shapes[idx].area * count)
            .sum()
}
