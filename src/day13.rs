use std::collections::{VecDeque, HashMap};
use crate::day13::Instruction::{Add1, Multiply2, Input3, Output4, JumpIfTrue5, JumpIfFalse6, LessThan7, Equals8, Halt99, RelativeBaseOffset9};
use crate::day13::ParameterMode::{PositionMode0, ImmediateMode1, RelativeMode2};
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::iter::once;
use std::thread::sleep;
use std::time::Duration;

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

type ScreenLocation = (isize, usize);
type Tile = usize;

fn draw_tile(tile: Tile) -> &'static str {
    match tile {
        0 => " ", // empty
        1 => "|", // wall
        2 => "#", // block
        3 => "-", // horizontal paddle
        4 => "*", // ball
        otherwise => panic!(format!("invalid tile {}", otherwise)),
    }
}

struct Screen {
    tiles: HashMap<ScreenLocation, Tile>,
}

impl Screen {
    fn new() -> Self {
        Self { tiles: HashMap::new() }
    }

    fn update(&mut self, tiles: HashMap<ScreenLocation, Tile>) {
        self.tiles.extend(tiles);
    }

    // returns a list of rows of tiles
    fn tiles(&self) -> Vec<Vec<Tile>> {
        let maxx = self.tiles.iter().map(|((x, _y), _pos)| *x).max().unwrap();
        let maxy = self.tiles.iter().map(|((_x, y), _pos)| *y).max().unwrap();
        (0..=maxy).map(|y| {
            (0..=maxx).map(|x| {
                *(self.tiles.get(&(x, y)))
                    .ok_or_else(|| format!("couldn't find {},{}, screen is {:?}", x, y, self.tiles))
                    .unwrap()
            }).collect()
        }).collect()
    }

    fn score(&self) -> Option<usize> {
        self.tiles.get(&(-1, 0)).map(|score| *score)
    }
}

fn draw_screen(screen: &Screen) -> String {
    screen.tiles().iter().map(|row| {
        row.iter().map(|tile| draw_tile(*tile)).join("")
    }).chain(once(format!("score: {:?}", screen.score())))
      .join("\n")
}

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    let mut icc = IntCodeComputer::new(proggy);
    let tiles = icc.run_until_halt().iter().tuples()
        .map(|(x, y, tile_id)| ((*x as isize, *y as usize), *tile_id as usize)).collect::<HashMap<_, _>>();
    tiles.iter().filter(|(_pos, tile_id)| **tile_id == 2).count()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    let mut proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    // set memory address 0 to 2 for free play
    proggy[0] = "2".to_string();
    let mut icc = IntCodeComputer::new(proggy);
    let mut screen = Screen::new();
    let mut player = GamePlayer::new();
    loop {
        let (output, run_result) = icc.run_and_collect_all_output();
        let tiles = output.iter().tuples()
            .map(|(x, y, tile_id)| ((*x as isize, *y as usize), *tile_id as Tile)).collect();
        screen.update(tiles);
        println!("{}", draw_screen(&screen));
        match run_result {
            RunResult::NeedMoreInput => {
                // sleep between each move so you can see what's going on
                sleep(Duration::from_millis(5));
                icc.queue_input(player.get_next_move(&screen));
            }
            RunResult::Halt => break,
            _ => panic!("this should never happen"),
        }
    }
    123
}

struct GamePlayer {
    previous_ball_x: Option<usize>,
}

impl GamePlayer {
    fn new() -> Self {
        Self { previous_ball_x: None }
    }

    // returns paddle movement in intcode input form:
    // -1 => move to the left
    // 1 => move to the right
    // 0 => stay
    fn get_next_move(&mut self, screen: &Screen) -> i128 {
        let current_ball_x = position_of_ball(&screen).0;
        let previous_ball_x = self.previous_ball_x.replace(current_ball_x);
        let paddle_x = position_of_paddle(screen).0;
        match previous_ball_x {
            None => 0,
            Some(previous_ball_x) => {
                if previous_ball_x == current_ball_x {
                    panic!("i don't think the ball ever goes straight up and down")
                } else if previous_ball_x < current_ball_x {   // if the ball is moving to the right
                    if current_ball_x == paddle_x { // ...and the ball is directly on top of the paddle
                        1 // then move to the right to follow it
                    } else if current_ball_x + 1 == current_ball_x { //... and the ball will be on top of the paddle next turn
                        0 // then stay in the same place and let the ball go on top of the paddle
                    } else if paddle_x < current_ball_x { // ...and the ball is to the right of the paddle
                        1 // then move to the right to follow, and pray, because we probably won't be able to catch it
                    } else if paddle_x > current_ball_x {  // ...and the ball is to the left of the paddle
                        -1 // then move to the left to follow, and pray that we'll be able to catch it
                    } else {
                        panic!("shouldn't have gotten here")
                    }
                } else if previous_ball_x > current_ball_x {  // if the ball is moving to the left
                    if current_ball_x == paddle_x { // ...and the ball is directly on top of the paddle
                        -1 // then move to the left to follow it
                    } else if current_ball_x - 1 == current_ball_x { //... and the ball will be on top of the paddle next turn
                        0 // then stay in the same place and let the ball go on top of the paddle
                    } else if paddle_x < current_ball_x { // ...and the ball is to the right of the paddle
                        1 // then move to the right to follow, and pray, because we probably won't be able to catch it
                    } else if paddle_x > current_ball_x {  // ...and the ball is to the left of the paddle
                        -1 // then move to the left to follow, and pray that we'll be able to catch it
                    } else {
                        panic!("shouldn't have gotten here")
                    }
                } else {
                    panic!("shouldn't have gotten here")
                }
            }
        }
    }
}


fn position_of_paddle(screen :&Screen) -> (usize, usize) {
    let pos = screen.tiles.iter().filter(|(_pos, item)| **item == 3).next().unwrap().0;
    ((pos.0 as usize), pos.1)
}

fn position_of_ball(screen :&Screen) -> (usize, usize) {
    let pos = screen.tiles.iter().filter(|(_pos, item)| **item == 4).next().unwrap().0;
    ((pos.0 as usize), pos.1)
}
