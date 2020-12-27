use eyre::{eyre, Result};
use itertools::Itertools;

#[aoc(day25, part1)]
fn solve_part1(input: &str) -> Result<u64> {
    let (pk1, pk2) = input
        .split('\n')
        .map(|v| v.parse::<u64>().unwrap())
        .collect_tuple()
        .ok_or_else(|| eyre!("Must have exactly 2 lines"))?;

    let l1 = dbg!(loop_target(7, pk1));
    let l2 = loop_target(7, pk2);

    let mut key = 1;
    for _ in 0..l1 {
        key = run(key, pk2);
    }

    Ok(key)
}

fn loop_target(sub: u64, target: u64) -> usize {
    let mut value = 1;
    for l in 1..usize::MAX {
        value = run(value, sub);
        if value == target {
            return l;
        }
    }
    panic!("No enough loops possible");
}

fn run(val: u64, sub: u64) -> u64 {
    let val = val * sub;
    val % 20201227
}

#[aoc(day25, part2)]
fn solve_part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "5764801
17807724";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(solve_part1(SAMPLE)?, 14897079);

        Ok(())
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(SAMPLE), 0);
    }
}
