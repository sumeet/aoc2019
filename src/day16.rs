use std::iter::repeat;
use itertools::Itertools;

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> String {
    let num_phases = 100;

    let mut u8s : Vec<u32> = input.lines().next().unwrap().chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    for _phase in 0..num_phases {
        u8s = (1..=u8s.len()).map(|input_i| {
            let pattern = gen_pattern(input_i);
            let sum = u8s.iter().zip(pattern).map(|(u, p)| *u as i32 * p).sum::<i32>();
            get_ones_digit(sum)
        }).collect();
    }
    u8s.iter().map(|u| u.to_string()).join("")
}

fn get_ones_digit(n: i32) -> u32 {
    n.to_string().chars().last().unwrap().to_digit(10).unwrap()
}

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
fn gen_pattern(input_i: usize) -> impl Iterator<Item = i32> {
    // the direction stay to skip the first one
    let skip_one = 1;
    BASE_PATTERN.iter().flat_map(move |i| repeat(*i).take(input_i)).cycle().skip(skip_one)
}

#[test]
fn p1() {
    println!("{}", solve_part1("12345678"));
    println!("{}", solve_part1("80871224585914546619083218645595"));
}