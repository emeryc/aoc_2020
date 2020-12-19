use eyre::{Error, Result};
use std::str::FromStr;

#[aoc_generator(day5)]
fn generator(input: &str) -> Result<Vec<u64>> {
    input
        .split('\n')
        .map(|pass| Ok(pass.parse::<SeatID>()?.0))
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(passes: &[u64]) -> u64 {
    *passes.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(passes: &[u64]) -> u64 {
    let mut passes: Vec<_> = passes.to_vec();
    passes.sort_unstable();
    passes
        .iter()
        .skip(1)
        .fold(passes.first().unwrap(), |last, cur| {
            if last + 1 != *cur {
                last
            } else {
                cur
            }
        })
        .to_owned()
}

struct SeatID(u64);
impl FromStr for SeatID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SeatID(
            s.chars()
                .map(|d| match d {
                    'F' => 0,
                    'L' => 0,
                    'B' => 1,
                    'R' => 1,
                    _ => unreachable!(),
                })
                .fold(0, |acc, v| (acc << 1) | v),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_0() -> Result<()> {
        assert_eq!("FBFBBFFRLR".parse::<SeatID>()?.0, 357);

        Ok(())
    }
}
