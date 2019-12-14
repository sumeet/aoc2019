use std::collections::HashMap;
use itertools::Itertools;
use num::Integer;

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> usize {
    let reaction_by_output = input.trim().lines().map(|line| {
        let (inputs_str, output_str) = line.split("=>").collect_tuple().unwrap();
        let inputs = inputs_str.trim().split(",").map(|input_str| {
            let input_str = input_str.trim();
            let (parts_str, chemical_str) = input_str.split(" ").collect_tuple().unwrap();
            ReactionPart::new(parts_str.parse().unwrap(), chemical_str.into())
        });
        let output_str = output_str.trim();
        let (parts_str, chemical_str) = output_str.split(" ").collect_tuple().unwrap();
        let output = ReactionPart::new(parts_str.parse().unwrap(), chemical_str.into());
        (output.chemical.clone(), Reaction::new(inputs.collect(), output))
    }).collect();
    num_ore_required(&ReactionPart::new(1, "FUEL".into()), &reaction_by_output)
}

fn num_ore_required(target: &ReactionPart, reactions_list: &ReactionsList) -> usize {
    if target.chemical == "ORE" {
        return target.parts
    }

    let reaction = reactions_list.get(&target.chemical).unwrap();
    let (num_reactions, num_unused) = div_ceil_rem(target.parts, reaction.output.parts);
    println!("producing {} of {}", target.parts, target.chemical);
    let deonjida = reaction.inputs.iter().map(|input| {
        num_ore_required(input, reactions_list)
    }).sum::<usize>() * num_reactions;
    println!("required {} ORE to produce {:?}", deonjida, target);
    deonjida
}

fn div_ceil_rem(top: usize, bot: usize) -> (usize, usize) {
    let (mut quotient, remainder) = top.div_rem(&bot);
    if remainder > 0 {
        quotient += 1
    }
    (quotient, remainder)
}

// keyed by output
type ReactionsList = HashMap<String, Reaction>;

#[derive(Debug)]
struct ReactionPart {
    parts: usize,
    chemical: String,
}

impl ReactionPart {
    fn new(parts: usize, chemical: String) -> Self {
        Self { parts, chemical }
    }
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<ReactionPart>,
    output: ReactionPart,
}

impl Reaction {
    pub fn new(inputs: Vec<ReactionPart>, output: ReactionPart) -> Self {
        Self { inputs, output }
    }
}

#[test]
fn p1_a() {
    assert_eq!(solve_part1("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"), 31)
}

#[test]
fn p1_b() {
    assert_eq!(solve_part1("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"), 165)
}

#[test]
fn test_div_ceil_rem() {
    // suppose 9 ORE => 2 A
    // and we want just 3 A
    let (num_reactions_needed, num_extra) = div_ceil_rem(3, 2);
    assert_eq!((num_reactions_needed, num_extra), (2, 1));
}