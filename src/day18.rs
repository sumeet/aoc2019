//use gen_iter::GenIter;
use gen_iter::GenIter;
use itertools::Itertools;
#[allow(unused)]
use pathfinding::directed::dijkstra::dijkstra;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
struct PossibleMove {
    robot_index: usize,
    next_pos: (usize, usize),
    next_space_type: SpaceType,
}

impl PossibleMove {
    fn new(robot_index: usize, next_pos: (usize, usize), next_space_type: SpaceType) -> Self {
        Self {
            robot_index,
            next_pos,
            next_space_type,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Prefix {
    start: char,
    middle: BTreeSet<char>,
    end: char,
}

impl Prefix {
    fn one_move(start: char, end: char) -> Self {
        Prefix {
            start,
            end,
            middle: BTreeSet::new(),
        }
    }

    fn new(start: char, middle: BTreeSet<char>, end: char) -> Self {
        Prefix { start, middle, end }
    }
}

#[derive(Debug, Clone)]
struct Map {
    robot_poss: Vec<(usize, usize)>,
    remaining_keys: HashSet<char>,
    space_by_pos: HashMap<(usize, usize), SpaceType>,
    num_moves: usize,
    previously_visited_by_robot_i: Vec<HashSet<(usize, usize)>>,
    door_positions: HashMap<char, (usize, usize)>,
    current_prefix: Option<Prefix>,
}

impl Map {
    fn parse(map_str: &str) -> Self {
        let mut robot_poss = vec![];
        let mut space_by_pos = HashMap::new();
        let mut all_keys = HashSet::new();
        let mut door_positions = HashMap::new();
        for (y, line) in map_str.trim().lines().enumerate() {
            for (x, map_chr) in line.chars().enumerate() {
                let space_type = match map_chr {
                    '#' => None,
                    '.' => Some(SpaceType::Empty),
                    '@' => {
                        robot_poss.push((x, y));
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
        let previously_visited_by_robot_i = robot_poss.iter().map(|_| HashSet::new()).collect();

        println!("{:?}", robot_poss.len());

        Self {
            robot_poss,
            space_by_pos,
            remaining_keys: all_keys,
            num_moves: 0,
            previously_visited_by_robot_i,
            door_positions,
            current_prefix: None,
        }
    }

    fn possible_moves(&self) -> impl Iterator<Item = PossibleMove> + '_ {
        self.robot_poss
            .iter()
            .enumerate()
            .flat_map(move |(robot_i, current_pos)| {
                [(0, 1), (0, -1), (-1, 0), (1, 0)]
                    .iter()
                    .filter_map(move |dxdy| {
                        let next_pos = checked_add_pos(*current_pos, *dxdy)?;
                        if self.previously_visited_by_robot_i[robot_i].contains(&next_pos) {
                            return None;
                        }
                        let (_pos, space_type) = self
                            .space_by_pos
                            .get_key_value(&next_pos)
                            .map(|(pos, space_type)| (*pos, *space_type))?;
                        // can't go to doors, if this door was really open it wouldn't be here, we would've
                        // turned it into an empty space when we picked up the key
                        if space_type.is_door() {
                            return None;
                        }
                        Some(PossibleMove::new(robot_i, next_pos, space_type))
                    })
            })
    }

    fn is_done(&self) -> bool {
        self.remaining_keys.is_empty()
    }

    #[allow(unused)]
    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        GenIter(move || {
            let mut q = VecDeque::new();
            q.push_front(self.clone());
            while let Some(map) = q.pop_back() {
                for possible_move in map.possible_moves().collect_vec() {
                    let next_map = map.go(possible_move);
                    if next_map.is_none() {
                        continue;
                    }
                    let next_map = next_map.unwrap();
                    if let SpaceType::Key(_) = possible_move.next_space_type {
                        yield next_map;
                    } else {
                        q.push_back(next_map);
                    }
                }
            }
        })
    }

    fn go(&self, possible_move: PossibleMove) -> Option<Self> {
        //        println!("possible_move: {:?}", possible_move);
        //        println!("prefix: {:?}, robot_poss: {}, remaining_keys: {}, space_by_pos: {}, previously_visited_by_roboti: {}, door_positions: {}",
        //            self.current_prefix, self.robot_poss.len(), self.remaining_keys.len(), self.space_by_pos.len(),
        //                 self.previously_visited_by_robot_i.iter().map(|hm| hm.len()).sum::<usize>(),
        //            self.door_positions.len(),
        //        );

        let mut next_map = self.clone();
        next_map.num_moves += 1;
        next_map.robot_poss[possible_move.robot_index] = possible_move.next_pos;
        next_map.previously_visited_by_robot_i[possible_move.robot_index]
            .insert(possible_move.next_pos);

        match possible_move.next_space_type {
            SpaceType::Empty => {}
            SpaceType::Door(_) => panic!("we shouldn't filtered out doors in possible_moves()"),
            SpaceType::Key(c) => {
                // turn the key into an empty space
                *next_map
                    .space_by_pos
                    .get_mut(&possible_move.next_pos)
                    .unwrap() = SpaceType::Empty;

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
                next_map.previously_visited_by_robot_i[possible_move.robot_index].clear();

                // store the prefix information, so we can ignore less efficient searches
                next_map.current_prefix = Some(match next_map.current_prefix {
                    // from the puzzle input, the entrance is @
                    None => Prefix::one_move('@', c),
                    Some(Prefix {
                        start,
                        mut middle,
                        end,
                    }) => {
                        middle.insert(end);
                        Prefix::new(start, middle, c)
                    }
                });

                // if any prefixes match ours, and are faster, then bail out
                //                let prefix = next_map.current_prefix.as_ref().unwrap();
                //                let current_winner = next_map
                //                    .winner_by_prefix
                //                    .get(prefix)
                //                    .map(|cw| *cw)
                //                    .unwrap_or(999999);
                //                if next_map.num_moves >= current_winner {
                //                    //                    println!(
                //                    //                        "dying due to prefix (winner was {}) {:?}",
                //                    //                        current_winner, prefix
                //                    //                    );
                //                    return None;
                //                }
                //                self.winner_by_prefix
                //                    .insert(prefix.clone(), next_map.num_moves);
            }
        }

        Some(next_map)
    }

    #[allow(unused)]
    fn find_min_path(&mut self) -> Option<Self> {
        let mut winner_by_prefix: HashMap<Option<Prefix>, usize> = HashMap::new();

        let mut q = BinaryHeap::new();
        q.push(DijkstraWrapper::new(self.clone()));
        while let Some(dw) = q.pop() {
            for possible_move in dw.map.possible_moves() {
                let mut next_map = dw.map.go(possible_move);
                if next_map.is_none() {
                    continue;
                }

                let next_map = next_map.unwrap();

                if let SpaceType::Key(_) = possible_move.next_space_type {
                    // next maps where the space is a key will ALWAYS have a prefix, only the first one doesn't
                    let prefix = &next_map.current_prefix;
                    let current_winner = winner_by_prefix
                        .get(prefix)
                        .map(|cw| *cw)
                        .unwrap_or(9999999);
                    if next_map.num_moves > current_winner {
                        continue;
                    }
                    winner_by_prefix.insert(prefix.clone(), next_map.num_moves);
                }

                if next_map.is_done() {
                    return Some(next_map);
                } else {
                    q.push(DijkstraWrapper::new(next_map))
                }
            }
        }
        None
    }

    //    #[allow(unused)]
    //    fn find_min_path_recursive(&mut self) -> Option<Self> {
    //        self.possible_moves()
    //            .into_par_iter()
    //            .filter_map(|(next_pos, space_type)| {
    //                let mut next_map = self.go(next_pos, space_type)?;
    //                if next_map.is_done() {
    //                    Some(next_map)
    //                } else {
    //                    next_map.find_min_path()
    //                }
    //            })
    //            .min_by_key(|map| map.num_moves)
    //    }

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

#[derive(Clone, Debug)]
struct DijkstraWrapper {
    map: Map,
}

impl DijkstraWrapper {
    fn new(map: Map) -> Self {
        Self { map }
    }

    fn neighbors(&self) -> impl Iterator<Item = (Self, usize)> + '_ {
        self.map.neighbors().map(move |map| {
            let num_moves = map.num_moves;
            (DijkstraWrapper::new(map), num_moves)
        })
    }
}

impl Ord for DijkstraWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        // order flipped to make this reverse
        other
            .map
            .num_moves
            .cmp(&self.map.num_moves)
            .then_with(|| self.map.current_prefix.cmp(&other.map.current_prefix))
    }
}

impl PartialOrd for DijkstraWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for DijkstraWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.current_prefix.hash(state);
    }
}

impl PartialEq for DijkstraWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.map.current_prefix.eq(&other.map.current_prefix)
    }
}

impl Eq for DijkstraWrapper {}

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    return 123;
    let map = Map::parse(input);

    let dijkstra_wrapper = DijkstraWrapper::new(map);
    let (_path, count) = dijkstra(
        &dijkstra_wrapper,
        move |dw| {
            dw.neighbors()
                .map(|(neighbor, next_num_moves)| (neighbor, next_num_moves - dw.map.num_moves))
                .collect_vec()
                .into_iter()
        },
        move |dw| dw.map.is_done(),
    )
    .unwrap();
    count
    //println!("{:?}", map.neighbors().collect::<Vec<_>>());

    //    let paths = map.find_min_path();
    ////    paths.unwrap().num_moves

    //paths.iter().map(|path| path.num_moves).min().unwrap()
    //    let destinations = map.destinations_from_current_pos(HashSet::new());
    //    format!("{:?}", destinations)
}

fn change_map_for_part_two(original_map_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut orig_entrance_pos = None;
    let lines = original_map_input.trim().lines();
    let mut new_map_input = vec![];
    for (y, line) in lines.enumerate() {
        let mut line_vec = vec![];
        for (x, chr) in line.chars().enumerate() {
            if chr == '@' {
                orig_entrance_pos = Some((x, y))
            }
            line_vec.push(chr);
        }
        new_map_input.push(line_vec);
    }

    let orig_entrance_pos = orig_entrance_pos.ok_or("didn't find entrance")?;

    // original:            should become:
    // ...                  @#@
    // .@.                  ###
    // ...                  @#@

    // for the @s in the corners
    let dxdy_for_corners = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
    for dxdy in dxdy_for_corners.iter() {
        let (x, y) = checked_add_pos(orig_entrance_pos, *dxdy)
            .ok_or("original entrance wasn't sufficiently in the middle")?;
        new_map_input[y][x] = '@'
    }

    // for the #s in the cross pattern (clockwise)
    let dxdy_for_cross = [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)];
    for dxdy in dxdy_for_cross.iter() {
        let (x, y) = checked_add_pos(orig_entrance_pos, *dxdy)
            .ok_or("original entrance wasn't sufficiently in the middle")?;
        new_map_input[y][x] = '#'
    }

    Ok(new_map_input
        .iter()
        .map(|line| line.iter().collect::<String>())
        .join("\n"))

    //    let top_left_corner = new_map_input[0..=orig_entrance_pos.1]
    //        .iter()
    //        .map(|line| line[0..=orig_entrance_pos.0].iter().join(""))
    //        .join("\n");
    //    println!("{}", top_left_corner);
    //    println!("");
    //    let top_right_corner = new_map_input[0..=orig_entrance_pos.1]
    //        .iter()
    //        .map(|line| line[orig_entrance_pos.0..line.len()].iter().join(""))
    //        .join("\n");
    //    println!("{}", top_right_corner);
    //
    //    //    println!(
    //    //        "{}",
    //    //        new_map_input
    //    //            .into_iter()
    //    //            .map(|line| line.into_iter().join(""))
    //    //            .join("\n")
    //    //    );
    //    unimplemented!()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> usize {
    let input = change_map_for_part_two(input).unwrap();
    println!();
    println!("{}", input);
    let map = Map::parse(&input);

    let dijkstra_wrapper = DijkstraWrapper::new(map);
    let (_path, count) = dijkstra(
        &dijkstra_wrapper,
        move |dw| {
            dw.neighbors()
                .map(|(neighbor, next_num_moves)| (neighbor, next_num_moves - dw.map.num_moves))
                .collect_vec()
                .into_iter()
        },
        move |dw| dw.map.is_done(),
    )
    .unwrap();
    count
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
    assert_eq!(
        136,
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
