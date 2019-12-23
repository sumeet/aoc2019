//use gen_iter::GenIter;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Map {
    current_pos: (usize, usize),
    remaining_keys: HashSet<char>,
    space_by_pos: HashMap<(usize, usize), SpaceType>,
    num_moves: usize,
    previously_visited: HashSet<(usize, usize)>,
    door_positions: HashMap<char, (usize, usize)>,
    min_path_found: Arc<AtomicUsize>,
}

impl Map {
    fn parse(map_str: &str) -> Self {
        let mut current_pos = None;
        let mut space_by_pos = HashMap::new();
        let mut all_keys = HashSet::new();
        let mut door_positions = HashMap::new();
        for (y, line) in map_str.trim().lines().enumerate() {
            for (x, map_chr) in line.chars().enumerate() {
                let space_type = match map_chr {
                    '#' => None,
                    '.' => Some(SpaceType::Empty),
                    '@' => {
                        current_pos = Some((x, y));
                        Some(SpaceType::Empty)
                    }
                    c if c.is_ascii_lowercase() => {
                        all_keys.insert(c);
                        Some(SpaceType::Key(c))
                    }
                    c if c.is_ascii_uppercase() => {
                        // lowercase doornames too so they're easier to match with keynames
                        let c = c.to_ascii_lowercase();
                        door_positions.insert(c, (x, y));
                        Some(SpaceType::Door(c))
                    }
                    otherwise => panic!(format!("unexpected: {}", otherwise)),
                };
                if let Some(space_type) = space_type {
                    space_by_pos.insert((x, y), space_type);
                }
            }
        }
        let current_pos = current_pos.take().unwrap();
        Self {
            current_pos,
            space_by_pos,
            remaining_keys: all_keys,
            num_moves: 0,
            previously_visited: HashSet::new(),
            door_positions,
            min_path_found: Arc::new(AtomicUsize::new(999999)),
        }
    }

    fn possible_moves(&self) -> impl IntoParallelIterator<Item = ((usize, usize), SpaceType)> + '_ {
        [(0, 1), (0, -1), (-1, 0), (1, 0)]
            .into_par_iter()
            .filter_map(move |dxdy| {
                let next_pos = checked_add_pos(self.current_pos, *dxdy)?;
                if self.previously_visited.contains(&next_pos) {
                    return None;
                }
                let (pos, space_type) = self
                    .space_by_pos
                    .get_key_value(&next_pos)
                    .map(|(pos, space_type)| (*pos, *space_type))?;
                // can't go to doors, if this door was really open it wouldn't be here, we would've
                // turned it into an empty space when we picked up the key
                if space_type.is_door() {
                    return None;
                }
                Some((pos, space_type))
            })
    }

    fn is_done(&self) -> bool {
        self.remaining_keys.is_empty()
    }

    fn go(&self, next_pos: (usize, usize), space_type: SpaceType) -> Self {
        let mut next_map = self.clone();
        match space_type {
            SpaceType::Empty => {}
            SpaceType::Key(c) => {
                // turn the key into an empty space
                *next_map.space_by_pos.get_mut(&next_pos).unwrap() = SpaceType::Empty;

                // if we pick up a key here, it should have been in remaining_keys, otherwise there's
                // a bug in the program
                assert!(next_map.remaining_keys.remove(&c));
                // and then open da door
                let door_pos = next_map.door_positions.remove(&c);
                if let Some(door_pos) = door_pos {
                    *next_map.space_by_pos.get_mut(&door_pos).unwrap() = SpaceType::Empty;
                }
                // and then clear previously visited locations, we're gonna start afresh after
                // grabbing da key because we need to be able to go the other direction
                next_map.previously_visited.clear();
            }
            SpaceType::Door(_) => panic!("we shouldn't filtered out doors in possible_moves()"),
        }
        next_map.current_pos = next_pos;
        next_map.previously_visited.insert(next_map.current_pos);
        next_map.num_moves += 1;
        next_map
    }

    fn find_min_path(&mut self) -> Option<Self> {
        if self.num_moves >= self.min_path_found.load(Ordering::Relaxed) {
            return None;
        }

        self.possible_moves()
            .into_par_iter()
            .filter_map(|(next_pos, space_type)| {
                let mut next_map = self.go(next_pos, space_type);
                if next_map.num_moves >= next_map.min_path_found.load(Ordering::Relaxed) {
                    return None;
                }

                if next_map.is_done() {
                    let min = self
                        .min_path_found
                        .fetch_min(next_map.num_moves, Ordering::Relaxed);
                    println!("done: {} moves (min is {})", next_map.num_moves, min);
                    Some(next_map)
                } else {
                    next_map.find_min_path()
                }
            })
            .min_by_key(|map| map.num_moves)
    }

    //    fn find_all_paths(&self) -> Vec<Self> {
    //        let mut paths = vec![];
    //        for (next_pos, space_type) in self.possible_moves() {
    //            let next_map = self.go(next_pos, space_type);
    //            if next_map.is_done() {
    //                paths.push(next_map);
    //            } else {
    //                paths.extend_from_slice(&next_map.find_all_paths())
    //            }
    //        }
    //        paths
    //    }
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

#[derive(Clone, Copy, Debug)]
enum SpaceType {
    Empty,
    Key(char),
    Door(char),
}

impl SpaceType {
    //    fn is_key(&self) -> bool {
    //        match self {
    //            SpaceType::Key(_) => true,
    //            _ => false,
    //        }
    //    }

    fn is_door(&self) -> bool {
        match self {
            SpaceType::Door(_) => true,
            _ => false,
        }
    }
}

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    let mut map = Map::parse(input);

    // need this threadpool stuff to increase stack size
    let pool = ThreadPoolBuilder::new()
        .stack_size(1024 * 1024 * 1000)
        .breadth_first()
        .build()
        .unwrap();
    pool.install(|| {
        let paths = map.find_min_path();
        paths.map(|p| p.num_moves).unwrap()
    })

    //    let paths = map.find_min_path();
    //    paths.unwrap().num_moves

    //paths.iter().map(|path| path.num_moves).min().unwrap()
    //    let destinations = map.destinations_from_current_pos(HashSet::new());
    //    format!("{:?}", destinations)
}

#[test]
fn e1() {
    println!(
        "{:?}",
        solve_part1(
            "#########
#b.A.@.a#
#########",
        )
    )
}

#[test]
fn e2() {
    println!(
        "{:?}",
        solve_part1(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        )
    )
}

#[test]
fn e3() {
    println!(
        "{:?}",
        solve_part1(
            "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
        )
    )
}

#[test]
fn e4() {
    println!(
        "{:?}",
        solve_part1(
            "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
        )
    )
}
