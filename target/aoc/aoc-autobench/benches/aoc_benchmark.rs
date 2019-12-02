#[macro_use]
extern crate criterion;
extern crate aoc2019;
extern crate aoc_runner;

use aoc2019::*;
use aoc_runner::ArcStr;
use criterion::Criterion;
use criterion::Fun;
use std::fmt::Display;

#[inline]
fn black_box(t: &dyn Display) {
    criterion::black_box(t);
}

fn aoc_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2019/day1.txt"));

    
    let mut day1_part1 = Vec::new();

    
    {
        let runner = Factory::day1_part1(input_day1.clone())
            .expect("failed to generate input for (default)");
        let fun = Fun::new("(default)", move |b, _| b.iter(|| runner.bench(black_box)));
        day1_part1.push(fun);
    }

    c.bench_functions("Day1 - Part1", day1_part1, ());
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn input_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2019/day1.txt"));

    
}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
