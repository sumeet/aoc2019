extern crate aoc2019;
extern crate aoc_runner;

use aoc2019::*;
use std::time::Instant;
use aoc_runner::ArcStr;

fn main() {
    println!("AOC 2019");

    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2019/day1.txt"));

    
    {
        let start_time = Instant::now();

        match Factory::day1_part1(input_day1.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();

                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!("Day 1 - Part 1 : {}\n\tgenerator: {:?},\n\trunner: {:?}\n", result, (inter_time - start_time), (final_time - inter_time));
                    },
                    Err(e) => eprintln!("Day 1 - Part 1 : FAILED while running :\n{:#?}\n", e)
                }
            },
            Err(e) => eprintln!("Day 1 - Part 1 : FAILED while generating :\n{:#?}\n", e)
        }
    }
    {
        let start_time = Instant::now();

        match Factory::day1_part2(input_day1.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();

                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!("Day 1 - Part 2 : {}\n\tgenerator: {:?},\n\trunner: {:?}\n", result, (inter_time - start_time), (final_time - inter_time));
                    },
                    Err(e) => eprintln!("Day 1 - Part 2 : FAILED while running :\n{:#?}\n", e)
                }
            },
            Err(e) => eprintln!("Day 1 - Part 2 : FAILED while generating :\n{:#?}\n", e)
        }
    }
}