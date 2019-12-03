use std::collections::HashSet;

#[derive(Debug)]
pub enum Turn {
    Left(usize),
    Up(usize),
    Down(usize),
    Right(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
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
    (((p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2)) as f64).sqrt() as usize

}

#[cfg(test)]
pub mod test {
    use crate::day3::{map_coordinates, Turn};

    #[test]
    fn coordinates() {
        println!("{:?}", map_coordinates(&[Turn::Right(75), Turn::Up(30)]));
    }
}