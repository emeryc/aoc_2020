use anyhow::{Error, Result};
use std::{fs::File, path::PathBuf};
use std::{io::prelude::*, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Eq, PartialEq)]
struct PasswordEntry {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let len: u8 = self.password.chars().filter(|c| c == &self.letter).count() as u8;

        len >= self.min && len <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let s = self.password.as_bytes(); //assumption that we aren't using utf8 for this, based on glancing at input
        let min = s.get((self.min - 1) as usize);
        let max = s.get((self.max - 1) as usize);

        let l_byte: u8 = {
            let s = self.letter.to_string();
            *s.as_bytes()
                .get(0)
                .expect("Assuming that we are only dealing with ASCII")
        };

        match (min, max) {
            (Some(min), Some(max)) => (min == &l_byte) ^ (max == &l_byte),
            _ => false,
        }
    }
}

impl FromStr for PasswordEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        nom_parser::parse(s)
    }
}

mod nom_parser {
    use crate::PasswordEntry;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag,
        character::complete::char,
        character::complete::{alpha1, anychar, multispace0, one_of},
        combinator::recognize,
        multi::many0,
        multi::many1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    pub(crate) fn parse(s: &str) -> Result<PasswordEntry> {
        let (s, (min, max)) = range(s).map_err(|e| anyhow!("{:?}", e))?;
        let (s, letter) = l(s).map_err(|e| anyhow!("{:?}", e))?;
        let (_, password) = pw(s).map_err(|e| anyhow!("{:?}", e))?;

        let letter = letter
            .chars()
            .next()
            .ok_or_else(|| anyhow!("Should have a character here."))?;
        let password = password.to_string();

        Ok(PasswordEntry {
            min: min.parse()?,
            max: max.parse()?,
            letter,
            password,
        })
    }

    fn pw(s: &str) -> IResult<&str, &str> {
        terminated(recognize(many1(anychar)), multispace0)(s)
    }

    fn l(s: &str) -> IResult<&str, &str> {
        terminated(terminated(alpha1, tag(":")), multispace0)(s)
    }

    fn range(s: &str) -> IResult<&str, (&str, &str)> {
        terminated(separated_pair(decimal, tag("-"), decimal), multispace0)(s)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut file = File::open(args.input)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let passwords = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<PasswordEntry>())
        .collect::<Result<Vec<_>>>()?;

    println!(
        "valid passwords: {}",
        passwords.iter().filter(|pw| pw.is_valid()).count()
    );

    println!(
        "valid passwords: {}",
        passwords.iter().filter(|pw| pw.is_valid_2()).count()
    );

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example() -> Result<()> {
        let passwords = vec![
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            PasswordEntry {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];

        assert_eq!(passwords.iter().filter(|pw| pw.is_valid()).count(), 2);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let passwords = vec![
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            PasswordEntry {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];

        assert_eq!(passwords.iter().filter(|pw| pw.is_valid_2()).count(), 1);

        Ok(())
    }

    #[test]
    fn parse() -> Result<()> {
        assert_eq!(
            PasswordEntry::from_str("1-3 a: abcde")?,
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            }
        );
        Ok(())
    }
    use crate::nom_parser;

    #[test]
    fn nom_parse() -> Result<()> {
        assert_eq!(
            nom_parser::parse("1-3 a: abcde")?,
            PasswordEntry {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            }
        );
        Ok(())
    }
}
