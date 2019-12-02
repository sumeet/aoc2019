use std::ops::Div;
use std::convert::TryInto;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> usize {
    let masses = input.lines().map(|i| i.parse::<usize>().unwrap());
    masses.map(|mass| (mass / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    let masses = input.lines().map(|i| i.parse::<usize>().unwrap());
    masses.map(calculate_fuel).sum()
}

fn calculate_fuel(mass: usize) -> usize {
    let mass = mass as isize;
    let fuel_required = (mass / 3) - 2;
    if fuel_required > 0 {
        fuel_required as usize + calculate_fuel(fuel_required.try_into().unwrap())
    } else {
        0
    }
}

#[cfg(test)]
pub mod test {
    use crate::day1::calculate_fuel;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
