use itertools::Itertools;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    fn new(pos: [isize; 3]) -> Self {
        Self {
            pos,
            velocity: [0, 0, 0],
        }
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> usize {
    let mut moons = input.lines().map(move |line| {
        let joined = line.chars().filter(|c| c.is_digit(10) || *c == ',' || *c == '-').join("");
        let pos : (isize, isize, isize) = joined.split(",").map(|num_str| num_str.parse::<isize>().unwrap()).collect_tuple().unwrap();
        Moon::new([pos.0, pos.1, pos.2])
    }).collect_vec();

    let num_steps = 1000;
    for _ in 0..num_steps {
        // apply gravity on all moon combinations
        for (moon_1_index, moon_2_index) in (0..moons.len()).tuple_combinations() {
            let moon_1 = moons[moon_1_index].clone();
            let moon_2 = moons[moon_2_index].clone();

            for (pos_i, (pos_1, pos_2)) in moon_1.pos.iter().zip(moon_2.pos.iter()).enumerate() {
                // lesser one increases...
                if pos_1 < pos_2 {
                    moons[moon_1_index].velocity[pos_i] += 1;
                    moons[moon_2_index].velocity[pos_i] -= 1;
                } else if pos_1 > pos_2 {
                    moons[moon_1_index].velocity[pos_i] -= 1;
                    moons[moon_2_index].velocity[pos_i] += 1;
                }
            }
        }

        // apply velocity on each moon
        for moon in moons.iter_mut() {
            for (position, velocity) in moon.pos.iter_mut().zip(moon.velocity.iter()) {
                *position += *velocity
            }
        }
    }

    moons.iter().map(|moon| kinetic_energy(moon) * potential_energy(moon)).sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> usize {
    let mut moons = input.lines().map(move |line| {
        let joined = line.chars().filter(|c| c.is_digit(10) || *c == ',' || *c == '-').join("");
        let pos : (isize, isize, isize) = joined.split(",").map(|num_str| num_str.parse::<isize>().unwrap()).collect_tuple().unwrap();
        Moon::new([pos.0, pos.1, pos.2])
    }).collect_vec();

    let mut previous_states = HashSet::new();
    previous_states.insert(hash(&moons));
    println!("{:?}", moons);

    let mut count = 0;
    loop {
        // apply gravity on all moon combinations
        for (moon_1_index, moon_2_index) in (0..moons.len()).tuple_combinations() {
            let moon_1 = moons[moon_1_index].clone();
            let moon_2 = moons[moon_2_index].clone();

            for (pos_i, (pos_1, pos_2)) in moon_1.pos.iter().zip(moon_2.pos.iter()).enumerate() {
                // lesser one increases...
                if pos_1 < pos_2 {
                    moons[moon_1_index].velocity[pos_i] += 1;
                    moons[moon_2_index].velocity[pos_i] -= 1;
                } else if pos_1 > pos_2 {
                    moons[moon_1_index].velocity[pos_i] -= 1;
                    moons[moon_2_index].velocity[pos_i] += 1;
                }
            }
        }

        // apply velocity on each moon
        for moon in moons.iter_mut() {
            for (position, velocity) in moon.pos.iter_mut().zip(moon.velocity.iter()) {
                *position += *velocity
            }
        }
        count += 1;
        let hash = hash(&moons);
        if previous_states.contains(&hash) {
            break
        } else {
            println!("{:?}", moons);
            previous_states.insert(hash);
        }
    }
    count
}

fn potential_energy(moon: &Moon) -> usize {
   moon.pos.iter().map(|pos| pos.abs() as usize).sum()
}

fn kinetic_energy(moon: &Moon) -> usize {
    moon.velocity.iter().map(|velocity| velocity.abs() as usize).sum()
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn p1() {
    let s = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    println!("{:?}", solve_part1(s))
}

#[test]
fn p2() {
    let s = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    println!("{:?}", solve_part2(s))
}
