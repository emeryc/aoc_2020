use anyhow::{anyhow, Error, Result};
use std::fmt::Display;
use std::str::FromStr;

#[aoc_generator(day5)]
fn generator(input: &str) -> Result<Vec<BoardingPass>> {
    input
        .split('\n')
        .map(|pass| pass.parse::<BoardingPass>())
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(passes: &[BoardingPass]) -> BoardingPass {
    passes.iter().max_by_key(|p| p.id()).unwrap().clone()
}

#[aoc(day5, part2)]
fn solve_part2(passes: &[BoardingPass]) -> BoardingPass {
    let mut passes: Vec<_> = passes.to_vec();
    passes.sort_unstable_by_key(|pass| (pass.row, pass.column));
    let before_missing = passes
        .iter()
        .skip(1)
        .fold(passes.first().unwrap(), |last, pass| {
            let ordered = if last.column == 7 {
                last.row + 1 == pass.row && pass.column == 0
            } else {
                last.row == pass.row && last.column + 1 == pass.column
            };
            if ordered {
                pass
            } else {
                last
            }
        });

    BoardingPass {
        row: {
            if before_missing.column == 7 {
                before_missing.row + 1
            } else {
                before_missing.row
            }
        },
        column: {
            if before_missing.column == 7 {
                0
            } else {
                before_missing.column + 1
            }
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
}

impl Display for BoardingPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "row: {}, column: {}: id: {}",
                self.row,
                self.column,
                self.id(),
            )
            .as_str(),
        )
    }
}

impl BoardingPass {
    fn id(&self) -> u64 {
        self.row as u64 * 8 + self.column as u64
    }
}

impl FromStr for BoardingPass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(anyhow!("Nothing to parse"));
        }
        let (r, c) = s.split_at(s.len() - 3);

        let (_, row) = r.chars().fold((0, 127), |(b, t), side| match side {
            'F' => (b, (b + t) / 2),
            'B' => ((b + t) / 2, t),
            _ => unreachable!(),
        });
        let (_, column) = c.chars().fold((0, 7), |(l, r), side| match side {
            'R' => ((l + r) / 2, r),
            'L' => (l, (l + r) / 2),
            _ => unreachable!(),
        });

        Ok(BoardingPass { row, column })
    }
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
        let p1 = "FBFBBFFRLR".parse::<BoardingPass>()?;
        assert_eq!(p1, BoardingPass { row: 44, column: 5 });
        assert_eq!(p1.id(), 357);
        assert_eq!("FBFBBFFRLR".parse::<SeatID>()?.0, 357);

        Ok(())
    }

    #[test]
    fn sample_1() -> Result<()> {
        let p1 = "BFFFBBFRRR".parse::<BoardingPass>()?;
        assert_eq!(p1, BoardingPass { row: 70, column: 7 });

        Ok(())
    }

    #[test]
    fn sample_2() -> Result<()> {
        let p1 = "FFFBBBFRRR".parse::<BoardingPass>()?;
        assert_eq!(p1, BoardingPass { row: 14, column: 7 });

        Ok(())
    }

    #[test]
    fn sample_3() -> Result<()> {
        let p1 = "BBFFBBFRLL".parse::<BoardingPass>()?;
        assert_eq!(
            p1,
            BoardingPass {
                row: 102,
                column: 4
            }
        );

        Ok(())
    }
}
