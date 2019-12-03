use std::collections::{HashSet, HashMap};
use std::convert::TryInto;

#[derive(Debug)]
pub enum Turn {
    Left(usize),
    Up(usize),
    Down(usize),
    Right(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn origin() -> Self {
        Point {x: 0, y: 0}
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let wire_paths : Vec<Vec<Turn>> = input.lines().map(|line| {
        line.split(",").filter(|s| !s.is_empty()).map(|s| {
            match s.chars().nth(0).unwrap() {
                'R' => {
                    Turn::Right(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'U' => {
                    Turn::Up(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'L' => {
                    Turn::Left(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'D' => {
                    Turn::Down(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                invalid => panic!(format!("invalid prefix: {}", invalid))
            }
        }).collect()
    }).collect();

    let first_wire_coordinates = map_coordinates(&wire_paths[0]);
    let second_wire_coordinates = map_coordinates(&wire_paths[1]);

    let intersection = first_wire_coordinates.intersection(&second_wire_coordinates);
    intersection.map(|intersection_point| {
        distance(Point::origin(), *intersection_point)
    }).min().unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let wire_paths : Vec<Vec<Turn>> = input.lines().map(|line| {
        line.split(",").filter(|s| !s.is_empty()).map(|s| {
            match s.chars().nth(0).unwrap() {
                'R' => {
                    Turn::Right(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'U' => {
                    Turn::Up(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'L' => {
                    Turn::Left(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                'D' => {
                    Turn::Down(s.chars().skip(1).collect::<String>().parse().unwrap())
                }
                invalid => panic!(format!("invalid prefix: {}", invalid))
            }
        }).collect()
    }).collect();

    let first_wire_coordinates = map_coordinates_with_steps(&wire_paths[0]);
    let second_wire_coordinates = map_coordinates_with_steps(&wire_paths[1]);

    let intersection = first_wire_coordinates.coords.intersection(
        &second_wire_coordinates.coords);
    intersection.map(|point| {
        let first_wire_steps = first_wire_coordinates.steps[point];
        let second_wire_steps = second_wire_coordinates.steps[point];
        first_wire_steps + second_wire_steps
    }).min().unwrap()
}

struct CoordsWithSteps {
    coords: HashSet<Point>,
    steps: HashMap<Point, usize>,
}

fn map_coordinates_with_steps(turns: &[Turn]) -> CoordsWithSteps {
    let mut seen_points = HashSet::new();
    let mut current_point = Point::origin();
    let mut steps_taken = HashMap::new();
    let mut num_steps_taken = 0;
    for turn in turns {
        match turn {
            Turn::Right(n) => {
                for _ in 0..*n {
                    current_point.x += 1;
                    num_steps_taken += 1;
                    seen_points.insert(current_point.clone());
                    steps_taken.insert(current_point.clone(), num_steps_taken);
                }
            }
            Turn::Left(n) => {
                for _ in 0..*n {
                    current_point.x -= 1;
                    num_steps_taken += 1;
                    seen_points.insert(current_point.clone());
                    steps_taken.insert(current_point.clone(), num_steps_taken);
                }
            }
            Turn::Up(n) => {
                for _ in 0..*n {
                    current_point.y += 1;
                    num_steps_taken += 1;
                    seen_points.insert(current_point.clone());
                    steps_taken.insert(current_point.clone(), num_steps_taken);
                }
            }
            Turn::Down(n) => {
                for _ in 0..*n {
                    current_point.y -= 1;
                    num_steps_taken += 1;
                    seen_points.insert(current_point.clone());
                    steps_taken.insert(current_point.clone(), num_steps_taken);
                }
            }
        }
    };
    CoordsWithSteps {
        coords: seen_points,
        steps: steps_taken,
    }
}

fn map_coordinates(turns: &[Turn]) -> HashSet<Point> {
    let mut seen_points = HashSet::new();
    let mut current_point = Point::origin();
    for turn in turns {
        match turn {
            Turn::Right(n) => {
                for _ in 0..*n {
                    current_point.x += 1;
                    seen_points.insert(current_point.clone());
                }
            }
            Turn::Left(n) => {
                for _ in 0..*n {
                    current_point.x -= 1;
                    seen_points.insert(current_point.clone());
                }
            }
            Turn::Up(n) => {
                for _ in 0..*n {
                    current_point.y += 1;
                    seen_points.insert(current_point.clone());
                }
            }
            Turn::Down(n) => {
                for _ in 0..*n {
                    current_point.y -= 1;
                    seen_points.insert(current_point.clone());
                }
            }
        }
    };
    seen_points
}

fn distance(p1: Point, p2: Point) -> usize {
    let d = (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
    d.try_into().unwrap()
}

#[cfg(test)]
pub mod test {
    use crate::day3::{map_coordinates, Turn};

    #[test]
    fn coordinates() {
        println!("{:?}", map_coordinates(&[Turn::Right(75), Turn::Up(30)]));
    }
}
