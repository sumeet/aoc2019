fn run_proggy(mut proggy: Vec<usize>, noun: usize, verb: usize) -> usize {
    proggy[1] = noun;
    proggy[2] = verb;
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

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    let proggy : Vec<usize> = input.lines().nth(0).unwrap().split(',')
        .map(|opcode_s| opcode_s.parse().unwrap()).collect();

    // 1202 protocol
    run_proggy(proggy, 12, 2)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    let proggy : Vec<usize> = input.lines().nth(0).unwrap().split(',')
        .map(|opcode_s| opcode_s.parse().unwrap()).collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_proggy(proggy.clone(), noun, verb);
            if output == 19690720 {
                return (100 * noun) + verb
            }
        }
    }
    panic!("try some different values")
}
