use num_rational::Ratio;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Slope {
    UpLeft(Ratio<isize>),
    UpRight(Ratio<isize>),
    DownLeft(Ratio<isize>),
    DownRight(Ratio<isize>),
    PosZero,
    NegZero,
    PosInfinity,
    NegInfinity,
}

impl Slope {
    fn calc(p1: &Point, p2: &Point) -> Self {
        let delta_x = p2.x as isize - p1.x as isize;
        let delta_y = p2.y as isize - p1.y as isize;
        if delta_x == 0 {
            return if delta_y > 0 {
                Slope::PosInfinity
            } else if delta_y < 0 {
                Slope::NegInfinity
            } else {
                panic!("can't calculate slope for the same point")
            }
        }
        if delta_y == 0 {
            return if delta_x > 0 {
                Slope::PosZero
            } else if delta_x < 0 {
                Slope::NegZero
            } else {
                panic!("can't calculate slope for the same point")
            }
        }
        match (delta_x, delta_y) {
            (x, y) if x < 0 && y < 0 => Slope::DownLeft(Ratio::new(delta_y, delta_x)),
            (x, y) if x < 0 && y > 0 => Slope::UpLeft(Ratio::new(delta_y, delta_x)),
            (x, y) if x > 0 && y < 0 => Slope::DownRight(Ratio::new(delta_y, delta_x)),
            (x, y) if x > 0 && y > 0 => Slope::UpRight(Ratio::new(delta_y, delta_x)),
            _ => panic!("wops"),
        }
    }
}

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

//fn angle(p1: &Point, p2: &Point) -> Option<isize> {
//    let delta_x = p2.x as isize - p1.x as isize;
//    if delta_x == 0 {
//        println!("None");
//        return None
//    }
//    let delta_y = p2.y as isize - p1.y as isize;
//    let f = Some(delta_y / delta_x);
//    println!("{:?}", f);
//    f
//}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let delta_y = p2.y as f64 - p1.y as f64;
    let delta_x = p2.x as f64 - p1.x as f64;
    (delta_y.powi(2) + delta_x.powi(2)).sqrt()
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
            Slope::calc(startpoint, endpoint)
        }).unique().count()
    }).max().unwrap()
//    let mut winner = 0;
//    for startpoint in asteroid_positions.iter() {
//        let mut points_by_slope = DefaultHashMap::new(vec![]);
//        let mut distances_by_slope = DefaultHashMap::new(vec![]);
//
//        for endpoint in asteroid_positions.iter().filter(|ep| *ep != startpoint) {
//            points_by_slope[Slope::calc(startpoint, endpoint)].push(endpoint);
//            let dist = (distance(startpoint, endpoint) * 100.) as usize;
//            distances_by_slope[Slope::calc(startpoint, endpoint)].push(dist);
//        }
//
//        let mut count = 0;
//        for (_, distances) in distances_by_slope.iter_mut() {
//            let mut count_by_distance = DefaultHashMap::new(0);
//            for distance in distances {
//                count_by_distance[distance] += 1
//            }
//            let min_distance = count_by_distance.keys().min().unwrap();
//            count += count_by_distance[min_distance];
//            if count_by_distance[min_distance] > 1 {
////                println!("min_distance: {:?}, count: {:?}", min_distance, count_by_distance[min_distance]);
//            }
//        }
//
////        if startpoint == &Point::new(5, 8) {
////            for (k, v) in points_by_slope.iter() {
////                println!("{:?}", k);
////                println!("\t{:?}", v);
////            }
////        }
//
//        let num_keys = count;
//        if num_keys > winner {
//            winner = num_keys;
//        } else {
//        }
//    }
//    winner
}

#[test]
fn p1() {
//    assert_eq!(solve_part1(".#..#
//.....
//#####
//....#
//...##"), 8);
//
//    assert_eq!(solve_part1("......#.#.
//#..#.#....
//..#######.
//.#.#.###..
//.#..#.....
//..#....#.#
//#..#....#.
//.##.#..###
//##...#..#.
//.#....####"), 33);
//
//    assert_eq!(solve_part1("#.#...#.#.
//.###....#.
//.#....#...
//##.#.#.#.#
//....#.#.#.
//.##..###.#
//..#...##..
//..##....##
//......#...
//.####.###."), 35);
//
//    assert_eq!(solve_part1(".#..##.###...#######
//##.############..##.
//.#.######.########.#
//.###.#######.####.#.
//#####.##.#.##.###.##
//..#####..#.#########
//####################
//#.####....###.#.#.##
//##.#################
//#####.##.###..####..
//..######..##.#######
//####.##.####...##..#
//.#####..#.######.###
//##...#.##########...
//#.##########.#######
//.####.#.###.###.#.##
//....##.##.###..#####
//.#.#.###########.###
//#.#.#.#####.####.###
//###.##.####.##.#..##"), 210);

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