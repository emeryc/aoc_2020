use std::collections::HashSet;

type GeneratorType = Instruction;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|line| {
            let mut s = line.split(' ');
            let action = s.next().unwrap();
            let value = s.next().map(|v| v.parse::<isize>().unwrap()).unwrap();
            match action {
                "nop" => Instruction::NOP(value),
                "acc" => Instruction::ACC(value),
                "jmp" => Instruction::JMP(value),
                _ => unreachable!(),
            }
        })
        .collect()
}

// Wrapper exists because of type stuff on this macro :(
#[aoc(day8, part1)]
fn solve_part1(input: &[GeneratorType]) -> isize {
    solve(input).unwrap_err()
}

fn solve(input: &[GeneratorType]) -> Result<isize, isize> {
    let mut counter: usize = 0;
    let mut acc = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        if visited.contains(&counter) {
            break Err(acc);
        }
        if input.len() == counter {
            break Ok(acc);
        }
        visited.insert(counter);
        match input[counter] {
            Instruction::NOP(_) => {}
            Instruction::ACC(delta) => acc += delta,
            Instruction::JMP(distance) => {
                counter = ((counter as isize) + distance) as usize;
                continue;
            }
        }
        counter += 1;
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &[GeneratorType]) -> isize {
    for (i, action) in input.iter().enumerate() {
        match action {
            Instruction::NOP(v) => {
                let mut input = input.to_vec();
                if let Some(elem) = input.get_mut(i) {
                    *elem = Instruction::JMP(*v);
                }
                if let Ok(acc) = solve(input.as_slice()) {
                    return acc;
                }
            }
            Instruction::ACC(_) => continue,
            Instruction::JMP(v) => {
                let mut input = input.to_vec();
                if let Some(elem) = input.get_mut(i) {
                    *elem = Instruction::NOP(*v);
                }
                if let Ok(acc) = solve(input.as_slice()) {
                    return acc;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 5);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 8);
    }
}
