use anyhow::{anyhow, Error, Result};
use std::{fs::File, path::PathBuf};
use std::{io::prelude::*, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut file = File::open(args.input)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let pass = input
        .split('\n')
        .filter_map(|pass| pass.parse::<BoardingPass>().ok())
        .max_by_key(|pass| pass.id())
        .ok_or_else(|| anyhow!("No passes?"))?;

    println!("{}", pass.id());

    let mut passes: Vec<_> = input
        .split('\n')
        .filter_map(|pass| pass.parse::<BoardingPass>().ok())
        .collect();
    passes.sort_unstable_by_key(|pass| (pass.row, pass.column));
    let before_missing = passes.iter().skip(1).fold(
        passes.first().ok_or_else(|| anyhow!("No First Element?"))?,
        |last, pass| {
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
        },
    );

    println!(
        "missing: {}",
        (BoardingPass {
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
        })
        .id()
    );

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_0() -> Result<()> {
        let p1 = "FBFBBFFRLR".parse::<BoardingPass>()?;
        assert_eq!(p1, BoardingPass { row: 44, column: 5 });
        assert_eq!(p1.id(), 357);

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
