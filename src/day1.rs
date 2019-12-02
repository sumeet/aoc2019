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
    let fuel_required = (mass / 3).checked_sub(2);
    match fuel_required {
        Some(0) | None => {
            0
        },
        Some(fuel_required) => fuel_required + calculate_fuel(fuel_required)
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
