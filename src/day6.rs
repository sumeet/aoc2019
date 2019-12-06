use defaultmap::DefaultHashMap;
use std::collections::{HashMap, HashSet};

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut direct_orbits : DefaultHashMap<&str, Vec<&str>> = DefaultHashMap::new(vec![]);

    for line in input.lines() {
        let mut or = line.split(")");
        let orbited = or.next().unwrap();;
        let orbiter = or.next().unwrap();;
        direct_orbits[orbiter].push(orbited);
    }

    let mut q = vec![];
    let mut count = 0;
    for (_orbiter, orbiteds) in direct_orbits.iter() {
        for orbited in orbiteds {
            q.push(orbited);
            count += 1;
        }
    }
    while !q.is_empty() {
        let orbiter = q.pop().unwrap();
        for orbited in direct_orbits[orbiter].iter() {
            q.push(orbited);
            count += 1
        }
    }
    count
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut direct_orbits : DefaultHashMap<&str, Vec<&str>> = DefaultHashMap::new(vec![]);
    let mut inverted_orbits : DefaultHashMap<&str, Vec<&str>> = DefaultHashMap::new(vec![]);

    for line in input.lines() {
        let mut or = line.split(")");
        let orbited = or.next().unwrap();;
        let orbiter = or.next().unwrap();;
        direct_orbits[orbiter].push( orbited);
        inverted_orbits[orbited].push(orbiter);
    }

    //print(&direct_orbits);
    find_shortest_path_to(&direct_orbits, &inverted_orbits, "YOU", "SAN", HashSet::new()).unwrap() - 2
}

#[allow(unused)]
fn print(dos: &HashMap<&str, &str>) {
    for (k ,v) in dos.iter() {
        println!("\"{}\" -> \"{}\";", k, v)
    }
}

fn find_shortest_path_to<'a>(direct_orbits: &'a DefaultHashMap<&'a str, Vec<&'a str>>, inverted_orbits: &'a DefaultHashMap<&'a str, Vec<&'a str>>, starting_point: &'a str, end_point: &str, mut history: HashSet<&'a str>) -> Option<usize> {
    history.insert(starting_point);
    let orbiters = &direct_orbits[starting_point];
    let orbitees = &inverted_orbits[starting_point];
    if orbiters.contains(&end_point) || orbitees.contains(&end_point) {
        Some(1)
    } else {
        orbiters.iter().chain(orbitees.iter()).filter_map(|new_starting_point| {
            if history.contains(*new_starting_point) {
                return None
            }
            find_shortest_path_to(direct_orbits, inverted_orbits, new_starting_point, end_point, history.clone())
        }).min().map(|p| p + 1)
    }
}


#[cfg(test)]
pub mod test {
    use crate::day6::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(solve_part1(input), 42)
    }

    #[test]
    fn test_part2() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(solve_part2(input), 4)
    }
}
