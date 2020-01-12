use gen_iter::GenIter;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Bug,
    Map(LazyMap),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LazyMap {
    Map(Map),
    None,
}

type Pos = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    this_map: BTreeMap<Pos, Space>,
    prev_map: Option<Box<Map>>,
}

impl Map {
    fn new(this_map: BTreeMap<Pos, Space>) -> Self {
        Self {
            this_map,
            prev_map: None,
        }
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
    Map::new(map)
}

fn parse_part2(input: &str) -> Map {
    let mut map = BTreeMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            match (x, y, space) {
                (2, 2, _) => map.insert((x, y), Space::Map(LazyMap::None)),
                (_, _, '#') => map.insert((x, y), Space::Bug),
                (_, _, '.') => map.insert((x, y), Space::Empty),
                otherwise => panic!(format!("didn't expect {:?}", otherwise)),
            };
        }
    }
    Map::new(map)
}

const ADJACENT_DXDYS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn adjacent_tiles(map: &Map, pos: Pos) -> impl Iterator<Item = Space> + '_ {
    GenIter(move || {
        for dxdy in &ADJACENT_DXDYS {
            let adj_pos = add_pos(pos, *dxdy);
            match adj_pos {
                (x, y) if x < 0 || y < 0 || x > 4 || y > 4 => {
                    yield Space::Empty;
                }
                (x, y) => {
                    let space = map.get(&(x as _, y as _)).cloned().unwrap();
                    match space {
                        Space::Map(LazyMap::None) => yield Space::Empty,
                        Space::Map(LazyMap::Map(inner_map)) => match square_number(pos) {
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
                                "shouldn't have gotten to an inner space from square {:?}",
                                otherwise
                            )),
                        },
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
            Space::Map(LazyMap::Map(inner_map)) => num_total_bugs(inner_map),
            Space::Map(LazyMap::None) => 0,
        })
        .sum()
}

fn num_adjacent_bugs(map: &Map, pos: Pos) -> usize {
    let adj_tiles = adjacent_tiles(map, pos);
    adj_tiles
        .map(|space| match space {
            Space::Empty => 0,
            Space::Bug => 1,
            Space::Map(_) => unimplemented!(),
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
                .map(|x| match map.get(&(x, y)).unwrap() {
                    Space::Map(_) => '?',
                    Space::Empty => '.',
                    Space::Bug => '#',
                })
                .join("")
        })
        .join("\n")
}

fn square_number(pos: Pos) -> u32 {
    let x = pos.0;
    let y = pos.1;
    (y as u32 * 5) + x as u32
}

fn biodiversity(map: &Map) -> usize {
    map.this_map
        .iter()
        .map(|(pos, space)| match space {
            Space::Empty => 0,
            Space::Map(_) => panic!("can't calculate biodiversity of recursive map"),
            Space::Bug => 2usize.pow(square_number(*pos)),
        })
        .sum()
}

fn generation(map: &Map) -> Map {
    let next_map = map
        .this_map
        .iter()
        .map(|(pos, space)| {
            let num_adj_bugs = num_adjacent_bugs(map, *pos);
            let space = match space {
                Space::Map(_) => unimplemented!(),
                Space::Bug => match num_adj_bugs {
                    1 => Space::Bug,
                    _ => Space::Empty,
                },
                Space::Empty => match num_adj_bugs {
                    1 | 2 => Space::Bug,
                    _ => Space::Empty,
                },
            };
            (*pos, space)
        })
        .collect();
    Map::new(next_map)
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let mut map = parse_part1(input);
    let mut seen_maps = HashSet::new();
    seen_maps.insert(map.clone());
    loop {
        map = generation(&map);
        if !seen_maps.insert(map.clone()) {
            return biodiversity(&map);
        }
    }
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
