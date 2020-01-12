//    0 1 2 3 4
// 0: 0 1 2 3 4
// 1: 5 6 7 8 9

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
type Map = BTreeMap<Pos, Space>;

fn parse_part1(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            match space {
                '#' => map.insert((x, y), Space::Bug),
                '.' => map.insert((x, y), Space::Empty),
                otherwise => panic!(format!("didn't expect {}", otherwise)),
            };
        }
    }
    map
}

fn parse_part2(input: &str) -> Map {
    let mut map = Map::new();
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
    map
}

const ADJACENT_DXDYS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn adjacent_tiles(map: &Map, pos: Pos) -> impl Iterator<Item = Space> + '_ {
    GenIter(move || {
        for dxdy in &ADJACENT_DXDYS {
            yield checked_add_pos(pos, *dxdy)
                .and_then(|adj_pos| map.get(&adj_pos))
                .cloned()
                .unwrap_or(Space::Empty);
        }
    })
}

fn num_total_bugs(map: &Map) -> usize {
    map.iter()
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

fn checked_add_pos(pos: (usize, usize), dxdy: (isize, isize)) -> Option<(usize, usize)> {
    let (dx, dy) = dxdy;
    Some((checked_add(pos.0, dx)?, checked_add(pos.1, dy)?))
}

fn checked_add(u: usize, i: isize) -> Option<usize> {
    if i < 0 {
        u.checked_sub(i.abs() as usize)
    } else {
        u.checked_add(i as usize)
    }
}

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

fn biodiversity(map: &Map) -> usize {
    map.iter()
        .map(|(pos, space)| match space {
            Space::Empty => 0,
            Space::Map(_) => panic!("can't calculate biodiversity of recursive map"),
            Space::Bug => {
                let x = pos.0;
                let y = pos.1;
                2usize.pow((y as u32 * 5) + x as u32)
            }
        })
        .sum()
}

fn generation(map: &Map) -> Map {
    map.iter()
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
        .collect()
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
