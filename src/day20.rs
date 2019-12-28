use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::once;

#[derive(Clone)]
struct Solver {
    map: Map,
    current_pos: Pos,
    previous_poss: HashSet<Pos>,
}

impl PartialEq for Solver {
    fn eq(&self, other: &Self) -> bool {
        self.current_pos == other.current_pos
    }
}

impl Eq for Solver {}

impl Hash for Solver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_pos.hash(state);
    }
}

impl Solver {
    fn new(map: Map) -> Self {
        let current_pos = map.entrance;
        Self {
            map,
            current_pos,
            previous_poss: HashSet::new(),
        }
    }

    fn is_done(&self) -> bool {
        self.current_pos == self.map.exit
    }

    fn possible_moves(&self) -> Vec<Solver> {
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
        new.previous_poss.insert(new.current_pos);
        new.current_pos = new_pos;
        new
    }
}

type Pos = (usize, usize);
type Dxdy = (isize, isize);

#[derive(Debug, Clone)]
struct Map {
    blank_spaces: HashSet<Pos>,
    // portals go both ways
    portals: HashMap<Pos, Pos>,
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

        let entrance = **entrance.unwrap();
        let exit = **exit.unwrap();
        Self {
            blank_spaces,
            portals,
            entrance,
            exit,
        }
    }
}

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let map = Map::parse(input);
    let solver = Solver::new(map);
    let dijkstra_result = dijkstra(
        &solver,
        |solver| {
            let cost = 1;
            let solvers = solver.possible_moves();
            solvers
                .into_iter()
                .map(|solver| (solver, cost))
                .collect::<Vec<(Solver, usize)>>()
        },
        |p| p.is_done(),
    );
    dijkstra_result.unwrap().1
}
