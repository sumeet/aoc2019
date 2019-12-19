use std::iter::repeat;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator,ParallelIterator};

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> String {
    let mut digits : Vec<u32> = input.lines().next().unwrap().chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    digits = fft(digits, 100);
    render_digits(&digits)
}

// UGH, need to figure out the trick to make this faster :(
#[aoc(day16, part2)]
fn solve_part2(input: &str) -> String {
    let initial_input_signal : Vec<u32> = input.lines().next().unwrap().chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    let offset = initial_input_signal.iter().take(7).map(|s| s.to_string()).join("").parse::<usize>().unwrap();
    let repeated_input_signal = repeat_whole_iterator(initial_input_signal.iter(), 10000).cloned().skip(offset).collect_vec();
    let output_signal = cheap_fft(repeated_input_signal, 100);
    // adding one because the problem indicates offsets starting a 1 (number to skip) rather than by 0
    render_digits(&output_signal[0..8])
}

fn render_digits(ds: &[u32]) -> String {
    ds.iter().map(|u| u.to_string()).join("")
}

fn repeat_whole_iterator<T: Copy>(i: impl Iterator<Item = T>, n_times: usize) -> impl Iterator<Item = T> {
    let contents = i.collect_vec();
    let len = contents.len();
    repeat(contents).flatten().take(len * n_times)
}

// this only works because of the nature of the input
fn cheap_fft(mut digits: Vec<u32>, num_phases: usize) -> Vec<u32> {
    for phase in 0..num_phases {
        println!("phase {}", phase);
        digits = (0..digits.len()).into_par_iter().map(|i| {
            get_ones_digit(digits[i..digits.len()].iter().sum::<u32>() as i32)
        }).collect();
    }
    digits
}

fn fft(mut digits: Vec<u32>, num_phases: usize) -> Vec<u32> {
    for phase in 0..num_phases {
        println!("phase {}", phase);
        digits = (1..=digits.len()).into_par_iter().map(|input_i| {
            let pattern = gen_pattern(input_i);
            let sum = digits.iter().zip(pattern).map(|(u, p)| *u as i32 * p).sum::<i32>();
            get_ones_digit(sum)
        }).collect();
    }
    digits
}

fn get_ones_digit(n: i32) -> u32 {
    (n.abs() % 10) as u32
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

#[test]
fn p2() {
    println!("{}", solve_part2("03036732577212944063491565474664"));
}
