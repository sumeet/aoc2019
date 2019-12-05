use crate::day5_part1::Instruction::{Add1, Multiply2, Input3, Output4, Halt99};
use crate::day5_part1::ParameterMode::{PositionMode0, ImmediateMode1};

#[derive(Debug)]
enum Instruction {
    Add1(ParameterMode, ParameterMode, ParameterMode),
    Multiply2(ParameterMode, ParameterMode, ParameterMode),
    Input3,
    Output4(ParameterMode),
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
        } else if s.ends_with("4") {
            let s = format!("{:0>3}", s);
            let first_param_mode = ParameterMode::parse(s.chars().nth(0).unwrap());
            Output4(first_param_mode)
        } else if s == "3" {
            Input3
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


#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> String {
    let proggy : Vec<String> = input.lines().nth(0).unwrap().split(',').map(|s| s.to_owned()).collect();
    run_proggy(proggy)
}

fn run_proggy(mut proggy: Vec<String>) -> String {
    let mut current_pos = 0;
    loop {
        let instruction = Instruction::parse(&proggy[current_pos].to_string());
        match instruction {
            Add1(first_mode, second_mode, third_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                let param_3 = get_third_param(&proggy, current_pos, third_mode);
                proggy[param_3 as usize] = (param_1 + param_2).to_string();
                current_pos += 4;
            },
            Multiply2(first_mode, second_mode, third_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                let param_3 = get_third_param(&proggy, current_pos, third_mode);
                proggy[param_3 as usize] = (param_1 * param_2).to_string();
                current_pos += 4;
            },
            Input3 => {
                let position = get_first_param(&proggy, current_pos, ImmediateMode1) as usize;
                // this is the only input instruction...
                proggy[position] = "1".to_owned();
                current_pos += 2;
            },
            Output4(mode) => {
                let param = get_first_param(&proggy, current_pos, mode);
                println!("{}", param);
                current_pos += 2;
            },
            Halt99 => {
                return proggy[0].clone();
            },
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
