use itertools::Itertools;

#[derive(Debug, Clone)]
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

//    fn set_pos(&mut self, i: usize, val: isize) {
//        self.pos[i] = val;
//    }

    fn set_velocity(&mut self, i: usize, val: isize) {
        self.velocity[i] = val;
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> usize {
    let mut moons = input.lines().map(move |line| {
        let joined = line.chars().filter(|c| c.is_digit(10) || *c == ',' || *c == '-').join("");
        let pos : (isize, isize, isize) = joined.split(",").map(|num_str| num_str.parse::<isize>().unwrap()).collect_tuple().unwrap();
        Moon::new([pos.0, pos.1, pos.2])
    }).collect_vec();

    //println!("{:?}", (0..moons.len()).tuple_combinations::<(usize, usize)>().collect_vec());

    let num_steps = 1;
    for _ in 0..num_steps {
        // apply gravity on all moon combinations
        for (moon_1_index, moon_2_index) in (0..moons.len()).tuple_combinations() {
            let moon_1 = moons[moon_1_index].clone();
            let moon_2 = moons[moon_2_index].clone();

            for (pos_i, (pos_1, pos_2)) in moon_1.pos.iter().zip(moon_2.pos.iter()).enumerate() {
                // lesser one increases...
                if pos_1 < pos_2 {
                    moons[moon_1_index].set_velocity(pos_i, pos_1 + 1);
                    moons[moon_2_index].set_velocity(pos_i, pos_2 - 1);
                } else if pos_1 > pos_2 {
                    moons[moon_1_index].set_velocity(pos_i, pos_1 - 1);
                    moons[moon_2_index].set_velocity(pos_i, pos_2 + 1);
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

    println!("{:?}", moons);
    moons.iter().map(|moon| kinetic_energy(moon) * potential_energy(moon)).sum()
}

fn potential_energy(moon: &Moon) -> usize {
   moon.pos.iter().map(|pos| pos.abs() as usize).sum()
}

fn kinetic_energy(moon: &Moon) -> usize {
    moon.velocity.iter().map(|velocity| velocity.abs() as usize).sum()
}

#[test]
fn p1() {
    let s = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    println!("{:?}", solve_part1(s))
}