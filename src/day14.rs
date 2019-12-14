use std::collections::HashMap;
use itertools::Itertools;
use defaultmap::DefaultHashMap;
use std::cmp::min;

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
    let mut waste = Waste::new();
    num_ore_required(&ReactionPart::new(1, "FUEL".into()),
        &reaction_by_output,
        &mut waste)
}

fn num_ore_required(target: &ReactionPart, reactions_list: &ReactionsList, waste: &mut Waste) -> usize {
    if target.chemical == "ORE" {
        return target.parts
    }

    let reaction = reactions_list.get(&target.chemical).unwrap();
    let num_pilfered_from_waste = waste.pilfer(&target.chemical, target.parts);
    let num_parts_needed = target.parts - num_pilfered_from_waste;

    let num_reactions = (num_parts_needed as f64 / reaction.output.parts as f64).ceil() as usize;
    let wasted_parts = (num_reactions * reaction.output.parts) - num_parts_needed;
    waste.add(target.chemical.to_string(), wasted_parts);
    reaction.inputs.iter().map(|input| {
        let input = ReactionPart::new(input.parts * num_reactions, input.chemical.clone());
        num_ore_required(&input, reactions_list, waste)
    }).sum()
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

#[derive(Debug)]
struct Waste {
    num_parts_by_chemical: DefaultHashMap<String, usize>,
}

impl Waste {
    fn new() -> Self {
        Self { num_parts_by_chemical: DefaultHashMap::new(0) }
    }

    fn add(&mut self, chemical: String, num_parts: usize) {
        self.num_parts_by_chemical[chemical] += num_parts
    }

    fn pilfer(&mut self, chemical: &str, num_parts_needed: usize) -> usize {
        let num_parts_in_waste = &mut self.num_parts_by_chemical[chemical.into()];
        let num_parts_to_take_from_waste = min(num_parts_needed, *num_parts_in_waste);
        *num_parts_in_waste -= num_parts_to_take_from_waste;
        num_parts_to_take_from_waste
    }
}

#[test]
fn p1_a() {
    assert_eq!(solve_part1("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"), 31);
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
fn p1_c() {
    assert_eq!(solve_part1("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312)
}
