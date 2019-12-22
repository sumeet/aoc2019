use std::collections::{VecDeque, HashSet, HashMap};
use crate::day17::Instruction::{Add1, Multiply2, Input3, Output4, JumpIfTrue5, JumpIfFalse6, LessThan7, Equals8, Halt99, RelativeBaseOffset9};
use crate::day17::ParameterMode::{PositionMode0, ImmediateMode1, RelativeMode2};
use defaultmap::DefaultHashMap;
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Add1(ParameterMode, ParameterMode, ParameterMode),
    Multiply2(ParameterMode, ParameterMode, ParameterMode),
    Input3(ParameterMode),
    Output4(ParameterMode),
    JumpIfTrue5(ParameterMode, ParameterMode),
    JumpIfFalse6(ParameterMode, ParameterMode),
    LessThan7(ParameterMode, ParameterMode, ParameterMode),
    Equals8(ParameterMode, ParameterMode, ParameterMode),
    RelativeBaseOffset9(ParameterMode),
    Halt99,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let parsed = if s.ends_with("1") {
            let s = format!("{:0>5}", s);
            let third_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let second_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(2).unwrap());
            Add1(first_param_mode, second_param_mode, third_param_mode)
        } else if s.ends_with("2") {
            let s = format!("{:0>5}", s);
            let third_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let second_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(2).unwrap());
            Multiply2(first_param_mode, second_param_mode, third_param_mode)
        } else if s.ends_with("3") {
            let s = format!("{:0>3}", s);
            let first_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            Input3(first_param_mode)
        } else if s.ends_with("4") {
            let s = format!("{:0>3}", s);
            let first_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            Output4(first_param_mode)
        } else if s.ends_with("5") {
            let s = format!("{:0>4}", s);
            let second_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            JumpIfTrue5(first_param_mode, second_param_mode)
        } else if s.ends_with("6") {
            let s = format!("{:0>4}", s);
            let second_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            JumpIfFalse6(first_param_mode, second_param_mode)
        } else if s.ends_with("7") {
            let s = format!("{:0>5}", s);
            let third_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let second_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(2).unwrap());
            LessThan7(first_param_mode, second_param_mode, third_param_mode)
        } else if s.ends_with("8") {
            let s = format!("{:0>5}", s);
            let third_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            let second_param_mode = ParameterMode::parse(s.chars().nth(1).unwrap());
            let first_param_mode = ParameterMode::parse(s.chars().nth(2).unwrap());
            Equals8(first_param_mode, second_param_mode, third_param_mode)
        } else if s == "99" {
            Halt99
        } else if s.ends_with("9") {
            let s = format!("{:0>3}", s);
            let first_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            RelativeBaseOffset9(first_param_mode)
        } else {
            panic!(format!("unable to parse instruction {}", s))
        };
        parsed
    }
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode0,
    ImmediateMode1,
    RelativeMode2,
}

impl ParameterMode {
    fn parse(c: char) -> Self {
        match c {
            '0' => PositionMode0,
            '1' => ImmediateMode1,
            '2' => RelativeMode2,
            _ => panic!(format!("unable to parse param mode {:?}", c))
        }
    }
}

fn get_first_param(proggy: &Proggy, instruction_pos: usize, mode: ParameterMode, relative_base: i128) -> i128 {
    let i = proggy[instruction_pos + 1].parse().unwrap();
    match mode {
        PositionMode0 => {
            proggy[i as usize].parse().unwrap()
        },
        ImmediateMode1 => {
            i
        },
        RelativeMode2 => {
            proggy[(i + relative_base) as usize].parse::<i128>().unwrap()
        }
    }
}

fn get_second_param(proggy: &Proggy, instruction_pos: usize, mode: ParameterMode, relative_base: i128) -> i128 {
    let i = proggy[instruction_pos + 2].parse().unwrap();
    match mode {
        PositionMode0 => {
            proggy[i as usize].parse().unwrap()
        },
        ImmediateMode1 => {
            i
        },
        RelativeMode2 => {
            proggy[(i + relative_base) as usize].parse::<i128>().unwrap()
        }
    }
}

fn get_third_param(proggy: &Proggy, instruction_pos: usize, mode: ParameterMode, relative_base: i128) -> i128 {
    match mode {
        PositionMode0 => {
            proggy[instruction_pos + 3].parse().unwrap()
        },
        ImmediateMode1 => {
            panic!("invalid program, third param can't be immediate mode")
        },
        RelativeMode2 => {
            proggy[instruction_pos + 3].parse::<i128>().unwrap() + relative_base
        }
    }
}

type Proggy = DefaultHashMap<usize, String>;

#[derive(Clone)]
struct IntCodeComputer {
    proggy: Proggy,
    input: VecDeque<i128>,
    current_pos: usize,
    relative_base: i128,
}

#[derive(Debug)]
enum RunResult {
    NeedMoreInput,
    Output(i128),
    Halt,
}

impl IntCodeComputer {
    fn new(proggy: Vec<String>) -> Self {
        let proggy = DefaultHashMap::new_with_map(
            "0".to_owned(), proggy.into_iter().enumerate().collect());
        IntCodeComputer { proggy, input: VecDeque::new(), current_pos: 0, relative_base: 0 }
    }

    #[allow(unused)]
    fn queue_input(&mut self, input: i128) {
        self.input.push_front(input);
    }

    #[allow(unused)]
    fn run_until_halt(&mut self) -> Vec<i128> {
        let mut all_output = vec![];
        loop {
            match self.run_and_get_next() {
                RunResult::Output(output) => all_output.push(output),
                RunResult::Halt => break,
                otherwise => panic!("didn't expect non-output, but got {:?}", otherwise),
            }
        }
        all_output
    }

    #[allow(unused)]
    fn run_and_collect_all_output(&mut self) -> (Vec<i128>, RunResult) {
        let mut all_output = vec![];
        let mut result;
        loop {
            result = self.run_and_get_next();
            match result {
                RunResult::Output(output) => all_output.push(output),
                RunResult::NeedMoreInput | RunResult::Halt => break,
            }
        }
        (all_output, result)
    }

    fn run_and_get_next(&mut self) -> RunResult {
        self.run().next().unwrap()
    }

    fn get_input_param(&self, mode: ParameterMode) -> usize {
        let pos = self.get_first_param(ImmediateMode1);
        match mode {
            PositionMode0 => pos as usize,
            ImmediateMode1 => panic!("inputs not allowed to be in immediate mode"),
            RelativeMode2 => (pos + self.relative_base) as usize,
        }
    }

    fn get_first_param(&self, mode: ParameterMode) -> i128 {
        get_first_param(&self.proggy, self.current_pos, mode, self.relative_base)
    }

    fn get_second_param(&self, mode: ParameterMode) -> i128 {
        get_second_param(&self.proggy, self.current_pos, mode, self.relative_base)
    }

    fn get_third_param(&self, mode: ParameterMode) -> i128 {
        get_third_param(&self.proggy, self.current_pos, mode, self.relative_base)
    }

    fn run(&mut self) -> impl Iterator<Item = RunResult> + '_ {
        std::iter::from_fn(move || {
            loop {
                let instruction = Instruction::parse(&self.proggy[self.current_pos].to_string());
                match instruction {
                    Add1(first_mode, second_mode, third_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        let param_3 = self.get_third_param(third_mode);
                        self.proggy[param_3 as usize] = (param_1 + param_2).to_string();
                        self.current_pos += 4;
                    },
                    Multiply2(first_mode, second_mode, third_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        let param_3 = self.get_third_param(third_mode);
                        self.proggy[param_3 as usize] = (param_1 * param_2).to_string();
                        self.current_pos += 4;
                    },
                    Input3(mode) => {
                        let raw_position = self.get_input_param(mode);
                        match self.input.pop_back() {
                            Some(input) => {
                                self.proggy[raw_position] = input.to_string();
                                self.current_pos += 2;
                            }
                            None => return Some(RunResult::NeedMoreInput)
                        }
                    },
                    Output4(mode) => {
                        let param = self.get_first_param(mode);
                        self.current_pos += 2;
                        return Some(RunResult::Output(param))
                    },
                    Halt99 => {
                        return Some(RunResult::Halt);
                    },
                    JumpIfTrue5(first_mode, second_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        if param_1 != 0 {
                            self.current_pos = param_2 as usize;
                        } else {
                            self.current_pos += 3;
                        }
                    }
                    JumpIfFalse6(first_mode, second_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        if param_1 == 0 {
                            self.current_pos = param_2 as usize;
                        } else {
                            self.current_pos += 3;
                        }
                    }
                    LessThan7(first_mode, second_mode, third_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        let param_3 = self.get_third_param(third_mode);
                        self.proggy[param_3 as usize] = if param_1 < param_2 {
                            "1".to_owned()
                        } else {
                            "0".to_owned()
                        };
                        self.current_pos += 4;
                    }
                    Equals8(first_mode, second_mode, third_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        let param_2 = self.get_second_param(second_mode);
                        let param_3 = self.get_third_param(third_mode);
                        self.proggy[param_3 as usize] = if param_1 == param_2 {
                            "1".to_owned()
                        } else {
                            "0".to_owned()
                        };
                        self.current_pos += 4;
                    }
                    RelativeBaseOffset9(first_mode) => {
                        let param_1 = self.get_first_param(first_mode);
                        self.relative_base += param_1 as i128;
                        self.current_pos += 2;
                    }
                }

            }
        })
    }
}


struct Map {
    rows: Vec<Vec<char>>,
}

impl Map {
    fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows.len()).map(move |y| {
            // gotta reverse the rows because otherwise the map is upside down
            // inside out
            (0..self.rows[y].len()).map(move |x| {
                (x, y)
            })
        }).flatten()
    }
}

fn parse_map(s: String) -> Map {
    Map { rows: s.lines().rev().map(|line| line.chars().collect()).collect() }
}

fn cells_needed_for_intersect(pos: (usize, usize), map: &Map) -> Option<Vec<char>> {
    //   c
    //  dab
    //   e
    let points = [
        (0, 0), //  a
        (1, 0), //  b
        (0, 1), //  c
        (-1, 0), // d
        (0, -1), // e
    ].iter().map(|(dx, dy)| {
        Some((checked_add(pos.0, *dx)?, checked_add(pos.1, *dy)?))
    }).collect::<Option<Vec<_>>>()?;
    points.iter().map(|point| Some(*map.rows.get(point.1)?.get(point.0)?)).collect()
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

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    let mut icc = IntCodeComputer::new(proggy);
    let output = icc.run_until_halt();
    let map_str = output.iter().map(|o| char::from(*o as u8)).collect();
    println!("{}", map_str);
    let map = parse_map(map_str);
    map.points().filter(|point| {
        if let Some(cells) = cells_needed_for_intersect(*point, &map) {
            cells.iter().all(|cell| *cell == '#')
        } else {
            false
        }
    }).map(|(x, y)| x * y).sum()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct MoveWithGoodies {
    cmd: Move,
    previously_unseen_locations: HashSet<(usize, usize)>,
    next_pos: (usize, usize),
}

impl MoveWithGoodies {
    fn left(current_pos: (usize, usize)) -> Self {
        MoveWithGoodies {
            cmd: Move::TurnLeft,
            previously_unseen_locations: HashSet::new(),
            next_pos: current_pos,
        }
    }

    fn right(current_pos: (usize, usize)) -> Self {
        MoveWithGoodies {
            cmd: Move::TurnRight,
            previously_unseen_locations: HashSet::new(),
            next_pos: current_pos,
        }
    }

    fn forward(n: usize, previously_unseen_locations: HashSet<(usize, usize)>,
               next_pos: (usize, usize)) -> Self {
        MoveWithGoodies {
            cmd: Move::Forward(n),
            previously_unseen_locations,
            next_pos
        }
    }

    fn num_previously_unseen_locations(&self) -> usize {
        self.previously_unseen_locations.len()
    }
}

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
enum Move {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

#[derive(Debug, Clone)]
struct Solver {
    // TODO: this can be minimized to number of characters later
    unvisited_locations: HashSet<(usize, usize)>,
    all_moves_so_far: Vec<Move>,
    scaffold_locations: HashSet<(usize, usize)>,
    current_pos: (usize, usize),
    facing: Direction,
}

impl Solver {
    fn from_map(map: &Map) -> Self {
        let mut scaffold_locations = HashSet::new();

        let mut direction_set = false;
        let mut current_pos = (0, 0);
        let mut facing = Direction::Up;

        for (x, y) in map.points() {
            match map.rows[y][x] {
                '#' => {
                    scaffold_locations.insert((x, y));
                },
                '^' => {
                    facing = Direction::Up;
                    current_pos = (x, y);
                    direction_set = true;
                },
                'v' => {
                    facing = Direction::Down;
                    current_pos = (x, y);
                    direction_set = true;
                },
                '<' => {
                    facing = Direction::Left;
                    current_pos = (x, y);
                    direction_set = true;
                },
                '>' => {
                    facing = Direction::Right;
                    current_pos = (x, y);
                    direction_set = true;
                },
                '.' => (),
                otherwise => panic!("invalid map char: {}", otherwise),
            }
        }

        if !direction_set {
            panic!("direction should have been set")
        }

        // unvisited locations are everything excluding the current pos
        let unvisited_locations = scaffold_locations.clone();
        // append the current pos to scaffold locations, we're just already there
        scaffold_locations.insert(current_pos);

        Self {
            scaffold_locations,
            current_pos,
            facing,
            unvisited_locations,
            all_moves_so_far: vec![],
        }
    }

    fn shortest_path_touching_everything_at_least_once(&self) -> Option<Self>  {
        let mut solver = self.clone();
        loop {
            if solver.is_complete() {
                return Some(solver)
            }

            let next_move = solver.next_move()?;
            solver = solver.go(&next_move);
        }
    }

    fn is_complete(&self) -> bool {
        self.unvisited_locations.is_empty()
    }

    fn go(&self, m: &MoveWithGoodies) -> Self {
        let mut next = self.clone();
        next.current_pos = m.next_pos;
        next.unvisited_locations = self.unvisited_locations
            .difference(&m.previously_unseen_locations).cloned().collect();
        next.facing = match m.cmd {
            Move::Forward(_) => next.facing.clone(),
            Move::TurnLeft => self.facing.turn_left(),
            Move::TurnRight => self.facing.turn_right(),
        };
        next.all_moves_so_far.push(m.cmd.clone());
        next
    }

    fn turn_left(&self) -> MoveWithGoodies {
        MoveWithGoodies::left(self.current_pos)
    }

    fn turn_right(&self) -> MoveWithGoodies {
        MoveWithGoodies::right(self.current_pos)
    }

    fn forward(&self, pos: (usize, usize), from: Direction) -> Option<MoveWithGoodies> {
        let mut previously_unseen_locations = HashSet::new();
        let (d1x, d1y) = self.d1xd1y(from);
        (1..).into_iter().map(move |i| {
            let dxdy = (d1x * i, d1y * i);
            let next_pos = checked_add_pos(pos, dxdy)?;
            if !self.scaffold_locations.contains(&next_pos) {
                return None
            }
            if self.unvisited_locations.contains(&next_pos) {
                previously_unseen_locations.insert(next_pos);
            }
            Some(
                MoveWithGoodies::forward(i as usize,
                                         previously_unseen_locations.clone(),
                                         next_pos))
        }).while_some()
          .max_by_key(|move_with_goodies| {
              move_with_goodies.num_previously_unseen_locations()
          })
    }

    fn next_move(&self) -> Option<MoveWithGoodies> {
        // if we can move forward and pick up some unseen locations, then do it
        let forward_from_current_pos = self.forward(self.current_pos,
                                                    self.facing);
        if forward_from_current_pos.is_some() {
            return forward_from_current_pos
        }

        // otherwise we need to turn either left or right, figure out which one

        // for left
        if let Some(_) = self.forward(self.current_pos, self.facing.turn_left()) {
            return Some(self.turn_left());
        }

        if let Some(_) = self.forward(self.current_pos, self.facing.turn_right()) {
            return Some(self.turn_right());
        }

        None
    }

    fn d1xd1y(&self, direction: Direction) -> (isize, isize) {
        match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct WindowIndex {
    starting_indeces_by_path: DefaultHashMap<Vec<Move>, Vec<usize>>,
}

impl WindowIndex {
    fn new() -> Self {
        Self { starting_indeces_by_path: DefaultHashMap::new(vec![]) }
    }

    fn saw_window(&mut self, window: Vec<Move>, at: usize) {
        self.starting_indeces_by_path[window].push(at);
    }

    fn most_effective_compression_candidates(&self) -> impl Iterator<Item = (&[Move], &[usize])> {
        self.starting_indeces_by_path.iter()
            .map(|(moves, indices)| (moves.as_slice(), indices.as_slice()))
            // number of moves times number of places it occurs is kind of like an efficiency score,
            // i think technically it's a little different but this might be good enough
            //
            // and then multiply by -1 for descending
            .sorted_by_key(|(moves, indices)| moves.len() as isize * indices.len() as isize * -1)
    }
}


#[derive(Clone, Debug, PartialEq)]
enum CompressedPathElement {
    Uncompressed(Move),
    Compressed(Vec<Move>),
}

impl CompressedPathElement {
    fn as_move(&self) -> Option<Move> {
        if let Self::Uncompressed(m) = self {
            Some(*m)
        } else {
            None
        }
    }

    fn as_moves(&self) -> Option<&[Move]> {
        if let Self::Compressed(m) = self {
            Some(&m)
        } else {
            None
        }
    }
}

// we can only compress into 3 functions, A, B, and C
const NUM_FUNCTIONS : usize = 3;
fn compress(path: &[Move]) -> Option<Vec<CompressedPathElement>> {
    let mut compressed = path.iter().cloned()
        .map(|m| CompressedPathElement::Uncompressed(m)).collect_vec();

    for _ in 0..NUM_FUNCTIONS {
        // these are the MAGIC NUMBERS!
        let window_lengths = (2..=8).filter(|i| i % 2 == 0).rev();

        let mut window_index = WindowIndex::new();
        for window_length in window_lengths {

            // TODO: shouldn't need to collect this...
            let compressed_enumerated = compressed.iter().enumerate().collect_vec();
            let uncompressed_windows = compressed_enumerated
                .windows(window_length)
                .map(|window| {
                    let (starting_index, _) = window.first().unwrap();
                    (*starting_index, window.iter().map(|(_i, pe)| pe).collect_vec())
                })
                .filter(| (_i, window)| !contains_compressed(window.into_iter()));
            for (starting_index, window) in uncompressed_windows {
                let window = window.iter().filter_map(|cpe| cpe.as_move()).collect();
                window_index.saw_window(window, starting_index)
            }
        }

        let first_uncompressed_index = compressed.iter().enumerate().find(|(_i, cpe)| {
            if let CompressedPathElement::Uncompressed(_) = cpe {
                true
            } else {
                false
            }
        }).unwrap().0;
        let (moves, indices) = window_index.most_effective_compression_candidates().filter(|(_moves, indices)| {
            indices.contains(&first_uncompressed_index)
        }).next()?;

        let mut original = compressed.iter().enumerate().peekable();
        let mut new_compressed = vec![];
        while original.peek().is_some() {
            let (index, cpe) = original.next().unwrap();
            if indices.contains(&index) {
                for _ in 0..(moves.len() - 1) {
                    original.next();
                }
                new_compressed.push(CompressedPathElement::Compressed(moves.to_vec()))
            } else {
                new_compressed.push(cpe.clone())
            }
        }
        compressed = new_compressed;

        if !still_contains_uncompressed(&compressed) {
            return Some(compressed)
        }
    }
    println!("{:?}", compressed);
    return None
}

fn contains_compressed<'a>(mut compressed: impl Iterator<Item = &'a&'a&'a CompressedPathElement>) -> bool {
    compressed.any(|cpe| {
        if let CompressedPathElement::Compressed(_) = cpe {
            return true
        } else {
            return false
        }
    })
}

fn still_contains_uncompressed(compressed: &[CompressedPathElement]) -> bool {
    compressed.iter().any(|cpe| {
        if let CompressedPathElement::Uncompressed(_) = cpe {
            return true
        } else {
            return false
        }
    })
}

fn to_ascii(mv: &Move) -> String {
    match mv {
        Move::TurnLeft => "L".to_string(),
        Move::TurnRight => "R".to_string(),
        Move::Forward(n) => n.to_string(),
    }
}

fn turn_into_ascii_input(compressed_path_elements: &[CompressedPathElement]) -> String {
    let mut cpes = compressed_path_elements.into_iter()
        .map(|cpe| Some(cpe)).collect_vec();

    let mut main_routine = std::iter::repeat("").take(cpes.len()).collect_vec();
    let function_names = ["A", "B", "C"];
    let mut function_by_name : HashMap<&str, &[Move]> = HashMap::with_capacity(function_names.len());
    for function_name in function_names.iter() {
        let first = cpes.iter().flatten().next().unwrap();
        let positions = cpes.iter()
            .positions(|cpe| cpe.as_ref() == Some(first)).collect_vec();
        function_by_name.insert(function_name, first.as_moves().unwrap());
        for position in positions {
            main_routine[position] = function_name;
            cpes[position] = None;
        }
    }

    main_routine.iter().join(",") + "\n" + &function_names.iter().map(|function_name| {
        function_by_name.get(function_name).unwrap().iter().map(to_ascii).join(",")
    }).join("\n")
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> i128 {
    // make a version of the map from part1, before making the program again
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    let mut icc = IntCodeComputer::new(proggy);
    let output = icc.run_until_halt();
    let map_str = output.iter().map(|o| char::from(*o as u8)).collect();
    let map = parse_map(map_str);
    let solver = Solver::from_map(&map);

    // solve the maze
    let shortest_path = solver.shortest_path_touching_everything_at_least_once().unwrap();

    // then compress the path
    let compressed = compress(&shortest_path.all_moves_so_far).unwrap();

    let ascii_input = turn_into_ascii_input(&compressed);

    // ok now begin part 2
    let mut proggy = input.split(",").map(|s| s.to_owned()).collect_vec();
    // make the robot wake up by changing the first instruction from a 1 to 2
    assert_eq!(proggy[0], "1");
    proggy[0] = "2".into();
    let mut icc = IntCodeComputer::new(proggy);
    let show_camera_feed = "n";
    for c in (ascii_input + "\n" + show_camera_feed + "\n").chars() {
        icc.queue_input(c as i128)
    }
    let output = icc.run_until_halt();
    let (num_dust_collected, map_output) = output.split_last().unwrap();
    let _map_str = map_output.iter().map(|o| char::from(*o as u8)).collect::<String>();
    //println!("{}", map_str);
    *num_dust_collected
}
