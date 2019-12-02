#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut proggy : Vec<usize> = input.lines().nth(0).unwrap().split(',')
        .map(|opcode_s| opcode_s.parse().unwrap()).collect();

    // 1202 protocol
    proggy[1] = 12;
    proggy[2] = 2;

    let mut current_pos = 0;
    loop {
        match proggy[current_pos] {
            1 => {
                let s1 = proggy[current_pos + 1];
                let s2 = proggy[current_pos + 2];
                let dest = proggy[current_pos + 3];
                proggy[dest] = proggy[s1] + proggy[s2];
                current_pos += 4;
            },
            2 => {
                let s1 = proggy[current_pos + 1];
                let s2 = proggy[current_pos + 2];
                let dest = proggy[current_pos + 3];
                proggy[dest] = proggy[s1] * proggy[s2];
                current_pos += 4;
            },
            99 => return proggy[0],
            _ => panic!("invalid operation"),
        }
    }
}
