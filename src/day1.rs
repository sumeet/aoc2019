use std::ops::Div;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> usize {
    let masses = input.lines().map(|i| i.parse::<usize>().unwrap());
    masses.map(|mass| (mass / 3) - 2).sum()
}

