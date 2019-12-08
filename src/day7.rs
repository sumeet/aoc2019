use crate::day7::Instruction::{Add1, Multiply2, Input3, Output4, Halt99, JumpIfTrue5, JumpIfFalse6, LessThan7, Equals8};
use crate::day7::ParameterMode::{PositionMode0, ImmediateMode1};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
enum Instruction {
    Add1(ParameterMode, ParameterMode, ParameterMode),
    Multiply2(ParameterMode, ParameterMode, ParameterMode),
    Input3,
    Output4(ParameterMode),
    JumpIfTrue5(ParameterMode, ParameterMode),
    JumpIfFalse6(ParameterMode, ParameterMode),
    LessThan7(ParameterMode, ParameterMode, ParameterMode),
    Equals8(ParameterMode, ParameterMode, ParameterMode),
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
            Input3
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
}

impl ParameterMode {
    fn parse(c: char) -> Self {
        match c {
            '0' => PositionMode0,
            '1' => ImmediateMode1,
            _ => panic!(format!("unable to parse param mode {:?}", c))
        }
    }
}

fn get_first_param(proggy: &[String], instruction_pos: usize, mode: ParameterMode) -> isize {
    let i = proggy[instruction_pos + 1].parse().unwrap();
    match mode {
        PositionMode0 => {
            proggy[i as usize].parse().unwrap()
        },
        ImmediateMode1 => {
            i
        },
    }
}

fn get_second_param(proggy: &[String], instruction_pos: usize, mode: ParameterMode) -> isize {
    let i = proggy[instruction_pos + 2].parse().unwrap();
    match mode {
        PositionMode0 => {
            proggy[i as usize].parse().unwrap()
        },
        ImmediateMode1 => {
            i
        },
    }
}

fn get_third_param(proggy: &[String], instruction_pos: usize, mode: ParameterMode) -> isize {
    if let ParameterMode::ImmediateMode1 = mode {
        panic!("invalid program, third param can't be immediate mode")
    }
    proggy[instruction_pos + 3].parse().unwrap()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> isize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();

    // initial input signal is 0
    (0..5).permutations(5).map(|phase_settings| {
        let mut output_signal = 0;
        for phase_setting in &phase_settings {
            output_signal = run_amplifier(proggy.clone(), *phase_setting, output_signal);
        }
        output_signal
    }).max().unwrap()
}

struct IntCodeComputer {
    proggy: Vec<String>,
    input: VecDeque<isize>,
    current_pos: usize,
}

#[derive(Debug)]
enum RunResult {
    NeedMoreInput,
    Output(isize),
    Halt,
}

impl IntCodeComputer {
    fn new(proggy: Vec<String>) -> Self {
        IntCodeComputer { proggy, input: VecDeque::new(), current_pos: 0 }
    }

    fn queue_input(&mut self, input: isize) {
        self.input.push_front(input);
    }

    fn run_and_get_next(&mut self) -> RunResult {
        self.run().next().unwrap()
    }

    fn run(&mut self) -> impl Iterator<Item = RunResult> + '_ {
        std::iter::from_fn(move || {
            loop {
                let instruction = Instruction::parse(&self.proggy[self.current_pos].to_string());
                match instruction {
                    Add1(first_mode, second_mode, third_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        let param_3 = get_third_param(&self.proggy, self.current_pos, third_mode);
                        self.proggy[param_3 as usize] = (param_1 + param_2).to_string();
                        self.current_pos += 4;
                    },
                    Multiply2(first_mode, second_mode, third_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        let param_3 = get_third_param(&self.proggy, self.current_pos, third_mode);
                        self.proggy[param_3 as usize] = (param_1 * param_2).to_string();
                        self.current_pos += 4;
                    },
                    Input3 => {
                        let position = get_first_param(&self.proggy, self.current_pos, ImmediateMode1) as usize;
                        match self.input.pop_back() {
                            Some(input) => {
                                self.proggy[position] = input.to_string();
                                self.current_pos += 2;
                            }
                            None => return Some(RunResult::NeedMoreInput)
                        }
                    },
                    Output4(mode) => {
                        let param = get_first_param(&self.proggy, self.current_pos, mode);
                        self.current_pos += 2;
                        return Some(RunResult::Output(param))
                    },
                    Halt99 => {
                        return Some(RunResult::Halt);
                    },
                    JumpIfTrue5(first_mode, second_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        if param_1 != 0 {
                            self.current_pos = param_2 as usize;
                        } else {
                            self.current_pos += 3;
                        }
                    }
                    JumpIfFalse6(first_mode, second_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        if param_1 == 0 {
                            self.current_pos = param_2 as usize;
                        } else {
                            self.current_pos += 3;
                        }
                    }
                    LessThan7(first_mode, second_mode, third_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        let param_3 = get_third_param(&self.proggy, self.current_pos, third_mode);
                        self.proggy[param_3 as usize] = if param_1 < param_2 {
                            "1".to_owned()
                        } else {
                            "0".to_owned()
                        };
                        self.current_pos += 4;
                    }
                    Equals8(first_mode, second_mode, third_mode) => {
                        let param_1 = get_first_param(&self.proggy, self.current_pos, first_mode);
                        let param_2 = get_second_param(&self.proggy, self.current_pos, second_mode);
                        let param_3 = get_third_param(&self.proggy, self.current_pos, third_mode);
                        self.proggy[param_3 as usize] = if param_1 == param_2 {
                            "1".to_owned()
                        } else {
                            "0".to_owned()
                        };
                        self.current_pos += 4;
                    }
                }

            }
        })
    }
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> isize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();

    (5..=9).permutations(5).map(|phase_settings| {
        // amplifiers A through E
        let mut amplifiers = (0..5)
            .map(|_| IntCodeComputer::new(proggy.clone()))
            .collect::<Vec<_>>();

        // for feedback loop mode, set the phase setting as the first input instruction
        for (phase_setting, amplifier) in phase_settings.iter().zip(amplifiers.iter_mut()) {
            amplifier.queue_input(*phase_setting);
        }

        let mut output_signal = 0;
        let mut amplifiers_alive = amplifiers.len();
        while amplifiers_alive > 0 {
            for amp in amplifiers.iter_mut() {
                match amp.run_and_get_next() {
                    RunResult::NeedMoreInput => {
                        amp.queue_input(output_signal);
                        match amp.run_and_get_next() {
                            RunResult::Output(output) => output_signal = output,
                            _ => panic!("expected output"),
                        }
                    }
                    RunResult::Halt => {
                        amplifiers_alive -= 1;
                    }
                    _ => panic!("didn't expect this to happen")
                }
            }
        }
        output_signal
    }).max().unwrap()
}

// phase setting can be 0-4
// returns output signal
fn run_amplifier(proggy: Vec<String>, phase_setting: u8, input_signal: isize) -> isize {
    let mut comp = IntCodeComputer::new(proggy);
    comp.queue_input(phase_setting as isize);
    comp.queue_input(input_signal);
    let mut output = comp.run();
    match output.next() {
        Some(RunResult::Output(output)) => output,
        otherwise => panic!(format!("expected output, but got {:?}", otherwise))
    }
}


#[test]
fn p1() {
    assert_eq!(65210, solve_part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"));
    assert_eq!(54321, solve_part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"));
    assert_eq!(43210, solve_part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"));
}


#[test]
fn p2() {
    assert_eq!(139629729, solve_part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"));
}
