use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::once;

type Pos = (usize, usize);
type Dxdy = (isize, isize);

#[derive(Clone)]
struct SolverPart1 {
    map: &'static Map,
    current_pos: Pos,
}

impl PartialEq for SolverPart1 {
    fn eq(&self, other: &Self) -> bool {
        self.current_pos == other.current_pos
    }
}

impl Eq for SolverPart1 {}

impl Hash for SolverPart1 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_pos.hash(state);
    }
}

impl SolverPart1 {
    fn new(map: &'static Map) -> Self {
        let current_pos = map.entrance;
        Self { map, current_pos }
    }

    fn is_done(&self) -> bool {
        self.current_pos == self.map.exit
    }

    fn possible_moves(&self) -> Vec<SolverPart1> {
        let mut next_solvers = [(0, 1), (0, -1), (-1, 0), (1, 0)]
            .iter()
            .filter_map(|dxdy| {
                let next_pos = checked_add_pos(self.current_pos, *dxdy)?;
                if self.map.blank_spaces.contains(&next_pos) {
                    Some(self.go(next_pos))
                } else {
                    None
                }
            })
            .collect_vec();
        let dest_portal = self.map.portals.get(&self.current_pos);
        if let Some(dp) = dest_portal {
            next_solvers.push(self.go(*dp));
        }
        next_solvers
    }

    fn go(&self, new_pos: Pos) -> Self {
        let mut new = self.clone();
        new.current_pos = new_pos;
        new
    }
}

type Level = usize;

#[derive(Clone, Debug)]
struct SolverPart2 {
    map: &'static Map,
    current_level: Level,
    current_pos: Pos,
}

impl PartialEq for SolverPart2 {
    fn eq(&self, other: &Self) -> bool {
        self.current_pos == other.current_pos && self.current_level == other.current_level
    }
}

impl Eq for SolverPart2 {}

impl Hash for SolverPart2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_level.hash(state);
        self.current_pos.hash(state);
    }
}

impl SolverPart2 {
    fn new(map: &'static Map) -> Self {
        let current_pos = map.entrance;
        Self {
            map,
            current_level: 0,
            current_pos,
        }
    }

    fn is_done(&self) -> bool {
        self.current_level == 0 && self.current_pos == self.map.exit
    }

    fn possible_moves(&self) -> Vec<SolverPart2> {
        let mut next_solvers = [(0, 1), (0, -1), (-1, 0), (1, 0)]
            .iter()
            .filter_map(|dxdy| {
                let next_pos = checked_add_pos(self.current_pos, *dxdy)?;

                // outer portals are walls at level 0
                if self.current_level == 0 && self.map.outer_portals.contains(&next_pos) {
                    return None;
                }

                // the entrance and exit are walls at anything other than level 0
                if self.current_level != 0
                    && (self.map.entrance == next_pos || self.map.exit == next_pos)
                {
                    return None;
                }

                if !self.map.blank_spaces.contains(&next_pos) {
                    return None;
                }
                Some(self.go(next_pos, self.current_level))
            })
            .collect_vec();

        let dest_portal = self.map.portals.get(&self.current_pos);
        if let Some(dp) = dest_portal {
            let next_level = if self.map.inner_portals.contains(&self.current_pos) {
                self.current_level.checked_add(1).unwrap()
            } else if self.map.outer_portals.contains(&self.current_pos) {
                self.current_level.checked_sub(1).unwrap()
            } else {
                panic!(format!(
                    "portal {:?} wasn't in either inner or outer",
                    self.current_pos
                ))
            };
            next_solvers.push(self.go(*dp, next_level));
        }
        next_solvers
    }

    fn go(&self, next_pos: Pos, next_level: usize) -> Self {
        //println!("going to {:?} {:?}", next_pos, next_level);
        let mut new = self.clone();
        new.current_pos = next_pos;
        new.current_level = next_level;
        new
    }
}

#[derive(Debug, Clone)]
struct Map {
    blank_spaces: HashSet<Pos>,
    // portals go both ways
    portals: HashMap<Pos, Pos>,
    inner_portals: HashSet<Pos>,
    outer_portals: HashSet<Pos>,
    entrance: Pos,
    exit: Pos,
}

fn portal_text_directions() -> impl Iterator<Item = (Dxdy, Dxdy)> {
    // different formats. the . is always the portal location, and the text around is just the label

    // .BC
    once(((1, 0), (2, 0)))
        .chain(
            // BC.
            once(((-2, 0), (-1, 0))),
        )
        .chain(
            // .
            // B
            // C
            once(((0, 1), (0, 2))),
        )
        // B
        // C
        // .
        .chain(once(((0, -2), (0, -1))))
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

impl Map {
    fn parse(input_str: &str) -> Map {
        let lines: Vec<(usize, Vec<(usize, char)>)> = input_str
            .lines()
            .enumerate()
            .map(|(y, line)| (y, line.chars().enumerate().collect()))
            .collect();

        let blank_spaces = lines
            .iter()
            .flat_map(move |(y, line)| {
                line.iter()
                    .filter_map(move |(x, ch)| if ch == &'.' { Some((*x, *y)) } else { None })
            })
            .collect::<HashSet<Pos>>();

        let lines = &lines;
        let poss_by_label = blank_spaces
            .iter()
            .flat_map(move |pos| {
                portal_text_directions().filter_map(move |(dxdy1, dxdy2)| {
                    let pos1 = checked_add_pos(*pos, dxdy1)?;
                    let pos2 = checked_add_pos(*pos, dxdy2)?;
                    let label1 = lines.get(pos1.1)?.1.get(pos1.0)?.1;
                    let label2 = lines.get(pos2.1)?.1.get(pos2.0)?.1;
                    if let ('A'..='Z', 'A'..='Z') = (label1, label2) {
                        Some(((label1, label2), pos))
                    } else {
                        None
                    }
                })
            })
            .into_group_map();

        let mut entrance = None;
        let mut exit = None;

        let mut portals: HashMap<Pos, Pos> = HashMap::new();
        for (label, poss) in &poss_by_label {
            match (label, poss.as_slice()) {
                (('A', 'A'), [pos]) => {
                    entrance = Some(pos);
                }
                (('Z', 'Z'), [pos]) => {
                    exit = Some(pos);
                }
                (_, [portal_pos_a, portal_pos_b]) => {
                    portals.insert(**portal_pos_a, **portal_pos_b);
                    portals.insert(**portal_pos_b, **portal_pos_a);
                }
                (label, portals) => {
                    panic!(
                        "should only be a max of 2 locs per label but got {:?} {:?}",
                        label, portals
                    );
                }
            }
        }

        let (portal_min_x, portal_max_x) = portals
            .keys()
            .map(|(x, _y)| *x)
            .minmax()
            .into_option()
            .unwrap();
        let (portal_min_y, portal_max_y) = portals
            .keys()
            .map(|(_x, y)| *y)
            .minmax()
            .into_option()
            .unwrap();
        let outer_portals = portals
            .keys()
            .cloned()
            .filter(|pos| {
                pos.0 == portal_min_x
                    || pos.1 == portal_min_y
                    || pos.0 == portal_max_x
                    || pos.1 == portal_max_y
            })
            .collect::<HashSet<Pos>>();
        let inner_portals = portals
            .keys()
            .cloned()
            .filter(|pos| !outer_portals.contains(pos))
            .collect::<HashSet<Pos>>();

        let entrance = **entrance.unwrap();
        let exit = **exit.unwrap();
        Self {
            blank_spaces,
            portals,
            inner_portals,
            outer_portals,
            entrance,
            exit,
        }
    }
}

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let map = Map::parse(input);
    let map = Box::new(map);
    let solver = SolverPart1::new(Box::leak(map));
    let dijkstra_result = dijkstra(
        &solver,
        |solver| {
            let cost = 1;
            let solvers = solver.possible_moves();
            solvers
                .into_iter()
                .map(|solver| (solver, cost))
                .collect::<Vec<(SolverPart1, usize)>>()
        },
        |p| p.is_done(),
    );
    dijkstra_result.unwrap().1
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> usize {
    let map = Map::parse(input);
    let map = Box::new(map);
    let solver = SolverPart2::new(Box::leak(map));
    let dijkstra_result = dijkstra(
        &solver,
        |solver| {
            let cost = 1;
            let solvers = solver.possible_moves();
            solvers
                .into_iter()
                .map(|solver| (solver, cost))
                .collect::<Vec<(SolverPart2, usize)>>()
        },
        |p| p.is_done(),
    );
    dijkstra_result.unwrap().1
}

#[test]
fn test_p2_simple() {
    assert_eq!(
        solve_part2(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z      "
        ),
        26
    )
}

#[test]
fn test_p2() {
    assert_eq!(
        solve_part2(
            "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "
        ),
        396
    )
}
