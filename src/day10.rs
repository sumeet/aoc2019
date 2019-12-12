use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

//badboy from https://stackoverflow.com/a/27481611
fn angle(p1: &Point, p2: &Point) -> usize {
    let delta_y = p2.y as f32 - p1.y as f32;
    let delta_x = p2.x as f32 - p1.x as f32;
    let result = delta_y.atan2(delta_x).to_degrees();
    let float = if result < 0. {
        360f32.to_degrees() + result
    } else {
        result
    };
    (float * 100.) as usize
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let delta_y = p2.y as f64 - p1.y as f64;
    let delta_x = p2.x as f64 - p1.x as f64;
    (delta_y.powi(2) + delta_x.powi(2)).sqrt()
}

fn distance_as_int(p1: &Point, p2: &Point) -> usize {
    (distance(p1, p2) * 100.) as usize
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut asteroid_positions = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.trim().chars().enumerate() {
            if cell == '#' {
                asteroid_positions.push(Point::new(x, y))
            } else if cell != '.' {
                panic!("invalid input")
            }
        }
    }

    asteroid_positions.iter().map(|startpoint| {
        asteroid_positions.iter()
            .filter(|endpoint| startpoint != *endpoint)
            .map(|endpoint| {
            angle(startpoint, endpoint)
        }).unique().count()
    }).max().unwrap()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut asteroid_positions = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.trim().chars().enumerate() {
            if cell == '#' {
                asteroid_positions.push(Point::new(x, y))
            } else if cell != '.' {
                panic!("invalid input")
            }
        }
    }

    let (startpoint, mut max_endpoints_by_angle) = asteroid_positions.iter().map(|startpoint| {
        (startpoint, asteroid_positions.iter()
            .filter(|endpoint| startpoint != *endpoint)
            .map(|endpoint| {
                (angle(startpoint, endpoint), endpoint)
            }).into_group_map())
    }).max_by_key(|(_startpoint, endpoints_by_angle)| endpoints_by_angle.keys().count()).unwrap();

    let mut endpoints_by_angle_increasing = max_endpoints_by_angle.iter_mut().map(|(angle, endpoints)| {
        endpoints.sort_by_key(|endpoint| distance_as_int(startpoint, endpoint));
        (scale_angle(*angle), endpoints)
    }).collect_vec();
    endpoints_by_angle_increasing.sort_by_key(|(scaled_angle, _endpoints)| *scaled_angle);

    let mut count = 0;
    loop {
        for (_angle, endpoints) in endpoints_by_angle_increasing.iter_mut() {
            if !endpoints.is_empty() {
                let vaporized_point = endpoints.remove(0);
                count += 1;
                if count == 200 {
                    return (vaporized_point.x * 100) + vaporized_point.y
                }
            }
        }
    }
}

fn scale_angle(angle: usize) -> u128 {
    // XXX: this is jank but it works, basically whatever angle calculation i'm using has this value straight up
    let directly_above_angle = 2053648;
    if angle >= directly_above_angle {
        return angle as u128
    } else {
       return angle as u128 + 9999999999
    }
}

#[test]
fn p1() {
    assert_eq!(solve_part1(".#..#
.....
#####
....#
...##"), 8);

    assert_eq!(solve_part1("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"), 33);

    assert_eq!(solve_part1("#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."), 35);

    assert_eq!(solve_part1(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"), 210);

    assert_eq!(solve_part1(".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."), 41)
}

#[test]
fn p2() {
    println!("{}", angle(&Point::new(8, 3), &Point::new(8, 1)));
    println!("{}", angle(&Point::new(8, 3), &Point::new(9, 0)));
    println!("{}", angle(&Point::new(8, 3), &Point::new(15, 1)));
    println!("{}", angle(&Point::new(8, 3), &Point::new(4, 4)));
    println!("{}", angle(&Point::new(8, 3), &Point::new(5, 1)));
}