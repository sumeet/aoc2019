use gen_iter::GenIter;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Bug,
    Map(InnerMap),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InnerMap {
    Map(Map),
    None,
}

type Pos = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    this_map: BTreeMap<Pos, Space>,
    parent_map: Option<Box<Map>>,
    level: isize,
}

impl Map {
    fn empty_with_empty_parent(level: isize) -> Self {
        Self::new(Self::empty_filling(), None, level)
    }

    fn new(this_map: BTreeMap<Pos, Space>, parent_map: Option<Box<Map>>, level: isize) -> Self {
        Self {
            this_map,
            parent_map,
            level,
        }
    }

    fn empty_filling() -> BTreeMap<Pos, Space> {
        let mut new_empty_map: BTreeMap<Pos, Space> = (0..5)
            .flat_map(|x| (0..5).map(move |y| ((x, y), Space::Empty)))
            .collect();
        new_empty_map.insert((2, 2), Space::Map(InnerMap::None));
        new_empty_map
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
    Map::new(map, None, 0)
}

fn parse_part2(input: &str) -> Map {
    let mut map = BTreeMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            match (x, y, space) {
                (2, 2, _) => map.insert((x, y), Space::Map(InnerMap::None)),
                (_, _, '#') => map.insert((x, y), Space::Bug),
                (_, _, '.') => map.insert((x, y), Space::Empty),
                otherwise => panic!(format!("didn't expect {:?}", otherwise)),
            };
        }
    }
    Map::new(map, Some(Box::new(Map::empty_with_empty_parent(-1))), 0)
}

const ADJACENT_DXDYS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn adjacent_tiles(map: &Map, pos: Pos) -> impl Iterator<Item = Space> + '_ {
    GenIter(move || {
        for dxdy in &ADJACENT_DXDYS {
            let adj_pos = add_pos(pos, *dxdy);
            match adj_pos {
                // off the left hand side, square number 12
                (x, _y) if x < 0 => {
                    if map.parent_map.is_none() {
                        yield Space::Empty;
                    } else {
                        yield map
                            .parent_map
                            .as_ref()
                            .unwrap()
                            .get(&(1, 2))
                            .cloned()
                            .unwrap()
                    }
                }
                // off the top, going to square #8
                (_x, y) if y < 0 => {
                    if map.parent_map.is_none() {
                        yield Space::Empty;
                    } else {
                        yield map
                            .parent_map
                            .as_ref()
                            .unwrap()
                            .get(&(2, 1))
                            .cloned()
                            .unwrap()
                    }
                }
                // off the right hand side, going to square #14
                (x, _y) if x > 4 => {
                    if map.parent_map.is_none() {
                        yield Space::Empty;
                    } else {
                        yield map
                            .parent_map
                            .as_ref()
                            .unwrap()
                            .get(&(3, 2))
                            .cloned()
                            .unwrap()
                    }
                }
                // off bottom side, going to square #18
                (_x, y) if y > 4 => {
                    if map.parent_map.is_none() {
                        yield Space::Empty;
                    } else {
                        yield map
                            .parent_map
                            .as_ref()
                            .unwrap()
                            .get(&(2, 3))
                            .cloned()
                            .unwrap()
                    }
                }
                (x, y) => {
                    let space = map.get(&(x as _, y as _)).cloned().unwrap();
                    match space {
                        Space::Map(InnerMap::None) => yield Space::Empty,
                        Space::Map(InnerMap::Map(inner_map)) => match square_number(pos) {
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
                                otherwise, pos
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
            Space::Map(InnerMap::Map(inner_map)) => num_total_bugs(inner_map),
            Space::Map(InnerMap::None) => 0,
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
    (y as u32 * 5) + x as u32 + 1
}

fn biodiversity(map: &Map) -> usize {
    map.this_map
        .iter()
        .map(|(pos, space)| match space {
            Space::Empty => 0,
            Space::Map(_) => panic!("can't calculate biodiversity of recursive map"),
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
            let space = match space {
                Space::Map(InnerMap::None) => Space::Map(InnerMap::Map(Map::new(
                    Map::empty_filling(),
                    Some(Box::new(map.clone())),
                    map.level + 1,
                ))),
                Space::Map(InnerMap::Map(inner_map)) => {
                    Space::Map(InnerMap::Map(generation(inner_map)))
                }
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
    Map::new(
        next_map,
        Some(Box::new(Map::empty_with_empty_parent(map.level - 1))),
        map.level,
    )
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

fn print_all_levels(mut map: &Map) {
    loop {
        println!("level {}", map.level);
        println!("{}", draw(map));
        println!();
        match map.get(&(2, 2)).unwrap() {
            Space::Map(InnerMap::Map(inner_map)) => {
                map = inner_map;
            }
            Space::Map(InnerMap::None) => {
                return;
            }
            otherwise => panic!("found a {:?} at pos 2,2", otherwise),
        }
    }
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let mut map = parse_part2(input);
    for _ in 0..2 {
        map = generation(&map);
    }
    print_all_levels(&map);
    num_total_bugs(&map)
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
