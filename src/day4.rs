#[aoc(day4, part1)]
pub fn solve_part1(_input: &str) -> usize {
    let mut num_solutions = 0;

    for i in 136818..685979 {
        let digits = digits(i);
        if contains_adjacent_digits(&digits) && only_increasing(&digits) {
            num_solutions += 1
        }
    }
    num_solutions
}

// we always have 8 characters
fn digits(i: usize) -> Vec<usize> {
    i.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn contains_adjacent_digits(digits: &[usize]) -> bool {
    digits.iter().enumerate().skip(1).any( |(i, d)| {
        *d == digits[i - 1]
    })
}

fn only_increasing(digits: &[usize]) -> bool {
    digits.is_sorted()
}

#[aoc(day4, part2)]
pub fn solve_part2(_input: &str) -> usize {
    (136818..685979).map(digits)
        .filter(|digits| contains_adjacent_digits_part_2(&digits))
        .filter(|digits| only_increasing(&digits))
        .map(|ds| dbg!(ds)).count()
}

fn contains_adjacent_digits_part_2(digits: &[usize]) -> bool {
    let mut groups = vec![];

    let mut group = vec![digits[0]];
    for d in digits.iter().skip(1) {
        if  *d == group[0] {
            group.push(*d);
        } else {
            groups.push(group.clone());
            group = vec![*d];
        }
    }
    groups.push(group.clone());
    groups.iter().any(|group| group.len() == 2)
}
