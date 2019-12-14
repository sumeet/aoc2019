use std::collections::HashMap;
use itertools::Itertools;

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
    let num_reactions = if target.parts <= reaction.output.parts {
        1
    } else {
        (target.parts as f64 / reaction.output.parts as f64).ceil() as usize
    };
    let deonjida = reaction.inputs.iter().map(|input| {
        num_ore_required(input, reactions_list)
    }).sum::<usize>() * num_reactions;
    println!("required {} ORE to produce {:?}", deonjida, target);
    deonjida
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
fn p1() {
    assert_eq!(solve_part1("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"), 31)
}