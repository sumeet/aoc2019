use crate::day25::Instruction::{
    Add1, Equals8, Halt99, Input3, JumpIfFalse6, JumpIfTrue5, LessThan7, Multiply2, Output4,
    RelativeBaseOffset9,
};
use crate::day25::ParameterMode::{ImmediateMode1, PositionMode0, RelativeMode2};
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::collections::VecDeque;

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
            _ => panic!(format!("unable to parse param mode {:?}", c)),
        }
    }
}

fn get_first_param(
    proggy: &Proggy,
    instruction_pos: usize,
    mode: ParameterMode,
    relative_base: i128,
) -> i128 {
    let i = proggy[instruction_pos + 1].parse().unwrap();
    match mode {
        PositionMode0 => proggy[i as usize].parse().unwrap(),
        ImmediateMode1 => i,
        RelativeMode2 => proggy[(i + relative_base) as usize]
            .parse::<i128>()
            .unwrap(),
    }
}

fn get_second_param(
    proggy: &Proggy,
    instruction_pos: usize,
    mode: ParameterMode,
    relative_base: i128,
) -> i128 {
    let i = proggy[instruction_pos + 2].parse().unwrap();
    match mode {
        PositionMode0 => proggy[i as usize].parse().unwrap(),
        ImmediateMode1 => i,
        RelativeMode2 => proggy[(i + relative_base) as usize]
            .parse::<i128>()
            .unwrap(),
    }
}

fn get_third_param(
    proggy: &Proggy,
    instruction_pos: usize,
    mode: ParameterMode,
    relative_base: i128,
) -> i128 {
    match mode {
        PositionMode0 => proggy[instruction_pos + 3].parse().unwrap(),
        ImmediateMode1 => panic!("invalid program, third param can't be immediate mode"),
        RelativeMode2 => proggy[instruction_pos + 3].parse::<i128>().unwrap() + relative_base,
    }
}

type Proggy = DefaultHashMap<usize, String>;

#[derive(Clone)]
struct IntCodeComputer {
    num_instructions_processed: usize,
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
        let proggy =
            DefaultHashMap::new_with_map("0".to_owned(), proggy.into_iter().enumerate().collect());
        IntCodeComputer {
            proggy,
            input: VecDeque::new(),
            current_pos: 0,
            relative_base: 0,
            num_instructions_processed: 0,
        }
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
        std::iter::from_fn(move || loop {
            let instruction = Instruction::parse(&self.proggy[self.current_pos].to_string());
            self.num_instructions_processed += 1;
            match instruction {
                Add1(first_mode, second_mode, third_mode) => {
                    let param_1 = self.get_first_param(first_mode);
                    let param_2 = self.get_second_param(second_mode);
                    let param_3 = self.get_third_param(third_mode);
                    self.proggy[param_3 as usize] = (param_1 + param_2).to_string();
                    self.current_pos += 4;
                }
                Multiply2(first_mode, second_mode, third_mode) => {
                    let param_1 = self.get_first_param(first_mode);
                    let param_2 = self.get_second_param(second_mode);
                    let param_3 = self.get_third_param(third_mode);
                    self.proggy[param_3 as usize] = (param_1 * param_2).to_string();
                    self.current_pos += 4;
                }
                Input3(mode) => {
                    let raw_position = self.get_input_param(mode);
                    match self.input.pop_back() {
                        Some(input) => {
                            self.proggy[raw_position] = input.to_string();
                            self.current_pos += 2;
                        }
                        None => return Some(RunResult::NeedMoreInput),
                    }
                }
                Output4(mode) => {
                    let param = self.get_first_param(mode);
                    self.current_pos += 2;
                    return Some(RunResult::Output(param));
                }
                Halt99 => {
                    return Some(RunResult::Halt);
                }
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
        })
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

#[aoc(day25, part1)]
fn solve_part1(input: &str) -> usize {
    let proggy: Vec<_> = input.split(",").map(|s| s.to_owned()).collect();
    let mut icc = IntCodeComputer::new(proggy);
    loop {
        let (output, _) = icc.run_and_collect_all_output();
        println!("{}", output.into_iter().map(|i| (i as u8) as char).join(""));
        let input: String = read_input();
        for chr in input.chars() {
            icc.queue_input(chr as i128);
        }
    }
    123
}
