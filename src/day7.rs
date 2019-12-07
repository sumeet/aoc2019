use crate::day7::Instruction::{Add1, Multiply2, Input3, Output4, Halt99, JumpIfTrue5, JumpIfFalse6, LessThan7, Equals8};
use crate::day7::ParameterMode::{PositionMode0, ImmediateMode1};
use std::iter::once;
use gen_iter::GenIter;

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


fn run_proggy(mut proggy: Vec<String>, mut input: impl Iterator<Item = isize>) -> Vec<isize> {
    let mut current_pos = 0;
    let mut output = vec![];
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
                proggy[position] = input.next().unwrap().to_string();
                current_pos += 2;
            },
            Output4(mode) => {
                let param = get_first_param(&proggy, current_pos, mode);
                output.push(param);
                current_pos += 2;
            },
            Halt99 => {
                return output;
            },
            JumpIfTrue5(first_mode, second_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                if param_1 != 0 {
                    current_pos = param_2 as usize;
                } else {
                    current_pos += 3;
                }
            }
            JumpIfFalse6(first_mode, second_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                if param_1 == 0 {
                    current_pos = param_2 as usize;
                } else {
                    current_pos += 3;
                }
            }
            LessThan7(first_mode, second_mode, third_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                let param_3 = get_third_param(&proggy, current_pos, third_mode);
                proggy[param_3 as usize] = if param_1 < param_2 {
                    "1".to_owned()
                } else {
                    "0".to_owned()
                };
                current_pos += 4;
            }
            Equals8(first_mode, second_mode, third_mode) => {
                let param_1 = get_first_param(&proggy, current_pos, first_mode);
                let param_2 = get_second_param(&proggy, current_pos, second_mode);
                let param_3 = get_third_param(&proggy, current_pos, third_mode);
                proggy[param_3 as usize] = if param_1 == param_2 {
                    "1".to_owned()
                } else {
                    "0".to_owned()
                };
                current_pos += 4;
            }
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

fn generate_all_phase_settings() -> impl Iterator<Item = [u8; 5]> {
    GenIter(move || {
        for i in 1..5 {
            let mut phase_setting = [99, 99, 99, 99, 99];
            phase_setting[0] = i;
            for j in (0..5).filter(move |ps| !phase_setting.contains(ps)) {
                phase_setting[1] = j;
                for k in (0..5).filter(move |ps| !phase_setting.contains(ps)) {
                    phase_setting[2] = k;
                    for l in (0..5).filter(move |ps| !phase_setting.contains(ps)) {
                        phase_setting[3] = l;
                        for m in (0..5).filter(move |ps| !phase_setting.contains(ps)) {
                            phase_setting[4] = m;
                            yield phase_setting;
                        }
                    }
                }
            }
        }
    })
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> isize {
    let proggy : Vec<_> = input.split(",").map(|s| s.to_owned()).collect();

    // initial input signal is 0
    let mut output_signal = 0;
    generate_all_phase_settings().map(move |phase_settings| {
        for phase_setting in &phase_settings {
            output_signal = run_amplifier(proggy.clone(), *phase_setting, output_signal)
        }
        output_signal
    }).max().unwrap()
}

// phase setting can be 0-4
// returns output signal
fn run_amplifier(proggy: Vec<String>, phase_setting: u8, input_signal: isize) -> isize {
    let input = once(phase_setting as isize)
        .chain(once(input_signal));
    run_proggy(proggy, input)[0]
}


#[test]
fn p1() {
    assert_eq!(65210, solve_part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"));
}
