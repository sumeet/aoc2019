use std::collections::{VecDeque, HashSet};
use crate::day15::Instruction::{Add1, Multiply2, Input3, Output4, JumpIfTrue5, JumpIfFalse6, LessThan7, Equals8, Halt99, RelativeBaseOffset9};
use crate::day15::ParameterMode::{PositionMode0, ImmediateMode1, RelativeMode2};
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

#[derive(Clone, Copy, PartialEq)]
enum TileState {
    Unknown,
    Empty,
    OxygenSystem,
    Wall,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

fn get_mapped_surroundings(map: &Map, around: &Position) -> [(Direction, TileState); 4] {
    let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    let mut surroundings = directions.iter()
        .map(|dir| (*dir, map[&from(around, *dir)]));
    [surroundings.next().unwrap(),
     surroundings.next().unwrap(),
     surroundings.next().unwrap(),
     surroundings.next().unwrap()]
}

type Position = (isize, isize);

fn from(pos: &Position, direction: Direction) -> Position {
    match direction {
        Direction::North => (pos.0, pos.1 + 1),
        Direction::South => (pos.0, pos.1 - 1),
        Direction::East => (pos.0 + 1, pos.1),
        Direction::West => (pos.0 - 1, pos.1),
    }
}

type Map = DefaultHashMap<Position, TileState>;

fn new_map() -> Map {
    let mut map = DefaultHashMap::new(TileState::Unknown);
    let initial_pos = (0, 0);
    // the robot starts on an empty position
    map[initial_pos] = TileState::Empty;
    map
}

#[allow(unused)]
fn render_map(map: &Map) -> String {
    let (minx, maxx) = map.keys().map(|pos| pos.0).minmax().into_option().unwrap();
    let (miny, maxy) = map.keys().map(|pos| pos.1).minmax().into_option().unwrap();
    (miny..=maxy).map(move |y| {
        (minx..=maxx).map(move |x| {
            if (x, y) == (0, 0) {
                return "O"
            }
            match map[&(x, y)] {
                TileState::Unknown => "?",
                TileState::Empty => ".",
                TileState::OxygenSystem => "X",
                TileState::Wall => "#",
            }
        }).join("")
    }).join("\n")
}

#[derive(Clone)]
struct Robot {
    icc: IntCodeComputer,
}

impl Robot {
    fn new(icc: IntCodeComputer) -> Self {
        Self { icc }
    }

    fn go(&self, direction: Direction) -> (TileState, Self) {
        let mut icc = self.icc.clone();
        let input = match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        };
        icc.queue_input(input);
        let (output, _) = icc.run_and_collect_all_output();
        if output.len() != 1 {
            panic!("there was a problem");
        }
        let output = output.first().unwrap();
        let tilestate_in_that_direction = match *output {
            0 => TileState::Wall,
            1 => TileState::Empty,
            2 => TileState::OxygenSystem,
            unknown => panic!(format!("got unknown tile state {}", unknown)),
        };
        (tilestate_in_that_direction, Robot::new(icc))
    }
}

fn explore_around(map: &mut Map, current_pos: Position, robot: Robot) {
    let mapped_surroundings = get_mapped_surroundings(map, &current_pos);
    let unknown_surroundings = mapped_surroundings.iter()
        .filter(|(_dir, tile_state)| *tile_state == TileState::Unknown)
        .map(|(dir, _tile_state)| dir);
    for direction in unknown_surroundings {
        let (tile_state, next_robot) = robot.go(*direction);
        let next_pos = from(&current_pos, *direction);
        map[next_pos] = tile_state;
        match tile_state {
            TileState::Unknown => panic!("this can't happen"),
            TileState::Empty | TileState::OxygenSystem => {
                // keep looking around and building the map!!!
                explore_around(map, next_pos, next_robot);
            },
            TileState::Wall => (), // do nothing, can't move past a wall
        }
    }
}

fn min_num_steps_to_oxygen(start: Position, map: &Map, mut visited: HashSet<Position>) -> Option<usize> {
    match map[&start] {
        TileState::Unknown | TileState::Wall => {
            // nowhere to go from here
            None
        },
        TileState::Empty => {
            get_mapped_surroundings(map, &start).iter()
                .filter_map(|(direction, _ts)| {
                    let next_pos = from(&start, *direction);
                    if visited.contains(&next_pos) {
                        return None
                    }
                    visited.insert(start);
                    min_num_steps_to_oxygen(next_pos, map, visited.clone())
                        .map(|num_steps| num_steps + 1)
                }).min()
        },
        TileState::OxygenSystem => Some(0),
    }
}


#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    let icc = IntCodeComputer::new(proggy);
    let robot = Robot::new(icc);;
    let mut map = new_map();
    explore_around(&mut map, (0, 0), robot);
    //println!("\n{}", render_map(&map));
    min_num_steps_to_oxygen((0, 0), &map, HashSet::new()).unwrap()
}

