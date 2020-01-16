use gen_iter::GenIter;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;

lazy_static! {
    static ref MAPS: Mutex<HashMap<isize, Map>> = Mutex::new(HashMap::new());
}

fn all_maps() -> Vec<Map> {
    let maps = MAPS.lock().unwrap();
    let mut all_maps = maps.values().cloned().collect_vec();
    all_maps.sort_by_key(|map| map.level);
    all_maps
}

fn get_map(i: isize) -> Map {
    let mut maps = MAPS.lock().unwrap();
    match maps.get(&i) {
        None => {
            {
                maps.insert(i, Map::empty(i));
                if !maps.contains_key(&(i - 1)) {
                    maps.insert(i - 1, Map::empty(i - 1));
                }
                if !maps.contains_key(&(i + 1)) {
                    maps.insert(i + 1, Map::empty(i + 1));
                }
            }
            Map::empty(i)
        }
        Some(map) => map.clone(),
    }
}

fn insert_map(map: Map) {
    let mut maps = MAPS.lock().unwrap();
    if !maps.contains_key(&(map.level - 1)) {
        maps.insert(map.level - 1, Map::empty(map.level - 1));
    }
    if !maps.contains_key(&(map.level + 1)) {
        maps.insert(map.level + 1, Map::empty(map.level + 1));
    }
    maps.insert(map.level, map);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Bug,
    InnerMap,
}

type Pos = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    this_map: BTreeMap<Pos, Space>,
    level: isize,
}

impl Map {
    fn empty(level: isize) -> Self {
        Self::new(Self::empty_filling(), level)
    }

    fn new(this_map: BTreeMap<Pos, Space>, level: isize) -> Self {
        Self { this_map, level }
    }

    fn empty_filling() -> BTreeMap<Pos, Space> {
        let mut new_empty_map: BTreeMap<Pos, Space> = (0..5)
            .flat_map(|x| (0..5).map(move |y| ((x, y), Space::Empty)))
            .collect();
        new_empty_map.insert((2, 2), Space::InnerMap);
        new_empty_map
    }

    fn inner_map(&self) -> Map {
        get_map(self.level + 1)
    }

    fn parent_map(&self) -> Map {
        get_map(self.level - 1)
    }

    fn get(&self, pos: &Pos) -> Option<&Space> {
        self.this_map.get(pos)
    }
}

fn parse_part1(input: &str) -> Map {
    let mut map = BTreeMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            match space {
                '#' => map.insert((x, y), Space::Bug),
                '.' => map.insert((x, y), Space::Empty),
                otherwise => panic!(format!("didn't expect {}", otherwise)),
            };
        }
    }
    Map::new(map, 0)
}

fn parse_part2(input: &str) -> Map {
    let mut map = BTreeMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            match (x, y, space) {
                (2, 2, _) => map.insert((x, y), Space::InnerMap),
                (_, _, '#') => map.insert((x, y), Space::Bug),
                (_, _, '.') => map.insert((x, y), Space::Empty),
                otherwise => panic!(format!("didn't expect {:?}", otherwise)),
            };
        }
    }

    Map::new(map, 0)
}

const ADJACENT_DXDYS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn adjacent_tiles(map: &Map, from_pos: Pos) -> impl Iterator<Item = Space> + '_ {
    GenIter(move || {
        for dxdy in &ADJACENT_DXDYS {
            let adj_pos = add_pos(from_pos, *dxdy);
            match adj_pos {
                // off the left hand side, square number 12
                (x, _y) if x < 0 => {
                    assert_eq!(12, square_number((1, 2)));
                    yield map.parent_map().get(&(1, 2)).cloned().unwrap()
                }
                // off the top, going to square #8
                (_x, y) if y < 0 => {
                    assert_eq!(8, square_number((2, 1)));
                    yield map.parent_map().get(&(2, 1)).cloned().unwrap()
                }
                // off the right hand side, going to square #14
                (x, _y) if x > 4 => {
                    assert_eq!(14, square_number((3, 2)));
                    yield map.parent_map().get(&(3, 2)).cloned().unwrap()
                }
                // off bottom side, going to square #18
                (_x, y) if y > 4 => {
                    assert_eq!(18, square_number((2, 3)));
                    yield map.parent_map().get(&(2, 3)).cloned().unwrap()
                }
                (x, y) => {
                    let space = map
                        .get(&(x as _, y as _))
                        .cloned()
                        .ok_or_else(|| {
                            format!(
                                "dying getting {},{} (lvl {}): Map:\n{}",
                                x,
                                y,
                                map.level,
                                draw(map)
                            )
                        })
                        .unwrap();
                    match space {
                        Space::InnerMap => {
                            let inner_map = map.inner_map();
                            match square_number(from_pos) {
                                8 => {
                                    // the top row
                                    for i in 0..5 {
                                        yield inner_map.get(&(i, 0)).cloned().unwrap();
                                    }
                                }
                                12 => {
                                    // the left column
                                    for i in 0..5 {
                                        yield inner_map.get(&(0, i)).cloned().unwrap();
                                    }
                                }
                                14 => {
                                    // the right column
                                    for i in 0..5 {
                                        yield inner_map.get(&(4, i)).cloned().unwrap();
                                    }
                                }
                                18 => {
                                    // the bottom row
                                    for i in 0..5 {
                                        yield inner_map.get(&(i, 4)).cloned().unwrap();
                                    }
                                }
                                otherwise => panic!(format!(
                                    "shouldn't have gotten to an inner space from square {:?} ({:?})",
                                    otherwise, from_pos
                                )),
                            }
                        }
                        _ => yield space,
                    }
                }
            }
        }
    })
}

fn num_total_bugs(map: &Map) -> usize {
    map.this_map
        .iter()
        .map(|(_, space)| match space {
            Space::Empty => 0,
            Space::Bug => 1,
            Space::InnerMap => 0,
        })
        .sum()
}

fn num_adjacent_bugs(map: &Map, pos: Pos) -> usize {
    let adj_tiles = adjacent_tiles(map, pos);
    adj_tiles
        .map(|space| match space {
            Space::Empty => 0,
            Space::Bug => 1,
            Space::InnerMap => panic!("this should never happen"),
        })
        .sum()
}

fn add_pos(pos: (usize, usize), dxdy: (isize, isize)) -> (isize, isize) {
    let (dx, dy) = dxdy;
    (pos.0 as isize + dx, pos.1 as isize + dy)
}

#[allow(unused)]
fn draw(map: &Map) -> String {
    (0..5)
        .map(|y| {
            (0..5)
                .map(|x| match map.get(&(x, y)) {
                    Some(Space::InnerMap) => '?',
                    Some(Space::Empty) => '.',
                    Some(Space::Bug) => '#',
                    None => 'N',
                })
                .join("")
        })
        .join("\n")
}

fn square_number(pos: Pos) -> u32 {
    let x = pos.0;
    let y = pos.1;
    (y as u32 * 5) + x as u32 + 1
}

fn biodiversity(map: &Map) -> usize {
    map.this_map
        .iter()
        .map(|(pos, space)| match space {
            Space::Empty => 0,
            Space::InnerMap => panic!("can't calculate biodiversity of recursive map"),
            Space::Bug => 2usize.pow(square_number(*pos) - 1),
        })
        .sum()
}

fn generation(map: &Map) -> Map {
    let next_map = map
        .this_map
        .iter()
        .map(|(pos, space)| {
            let num_adj_bugs = num_adjacent_bugs(map, *pos);
            let next_space = match space {
                Space::InnerMap => Space::InnerMap,
                Space::Bug => match num_adj_bugs {
                    1 => Space::Bug,
                    _ => Space::Empty,
                },
                Space::Empty => match num_adj_bugs {
                    1 | 2 => Space::Bug,
                    _ => Space::Empty,
                },
            };
            (*pos, next_space)
        })
        .collect();
    Map::new(next_map, map.level)
}

//#[aoc(day24, part1)]
//fn solve_part1(input: &str) -> usize {
//    let mut map = parse_part1(input);
//    let mut seen_maps = HashSet::new();
//    seen_maps.insert(map.clone());
//    loop {
//        map = generation(&map);
//        if !seen_maps.insert(map.clone()) {
//            return biodiversity(&map);
//        }
//    }
//}

fn print_all_levels() {
    for map in all_maps() {
        println!("Level {}\n{}", map.level, draw(&map));
    }
}

fn generate_all_maps() {
    for map in all_maps() {
        let next_map = generation(&map);
        insert_map(next_map);
    }
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let map = parse_part2(input);
    insert_map(map);
    //    print_all_levels();
    //    println!();
    //    println!();
    //    println!();
    //    generate_all_maps();
    //    print_all_levels();
    //
    //
    //    print_all_levels();

    for _ in 0..200 {
        generate_all_maps();
    }
    all_maps().iter().map(num_total_bugs).sum()
}

#[test]
fn ex2() {
    let map = parse_part2(
        "....#
#..#.
#.?##
..#..
#....",
    );
    insert_map(map);
    for _ in 0..10 {
        generate_all_maps();
    }
    print_all_levels();
}

#[test]
fn example() {
    let map = parse_part1(
        "....#
#..#.
#..##
..#..
#....",
    );
    println!("{}", draw(&map));
    let map = generation(&map);
    println!();
    println!("{}", draw(&map));
    println!();
    let map = generation(&map);
    println!("{}", draw(&map));
    println!();
    let map = generation(&map);
    println!("{}", draw(&map));
}

#[test]
fn biodiversity_example() {
    let map = parse_part1(
        ".....
.....
.....
#....
.#...",
    );
    assert_eq!(biodiversity(&map), 2129920);
}

#[test]
fn example_2() {
    let map = parse_part2(
        "....#
#..#.
#.?##
..#..
#....",
    );
}
