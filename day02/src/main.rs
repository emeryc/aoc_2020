use anyhow::{anyhow, Context, Result};
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};
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

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.splitn(3, ' ');
        let err_msg = || anyhow!("Expected format to be 'i-i c: pw, got {}", s);
        let range = split.next().ok_or_else(err_msg)?;
        let mut rsplit = range.split('-');
        let min = rsplit
            .next()
            .ok_or_else(err_msg)?
            .parse()
            .with_context(|| format!("with input string - {}", s))?;
        let max = rsplit
            .next()
            .ok_or_else(err_msg)?
            .parse()
            .with_context(|| format!("with input string - {}", s))?;
        let letter = split.next().ok_or_else(err_msg)?;
        let password = split.next().ok_or_else(err_msg)?;

        Ok(PasswordEntry {
            min,
            max,
            letter: letter.chars().next().ok_or_else(err_msg)?,
            password: password.to_string(), // This is a un-needed allocation, but *shrug*
        })
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
        .map(|s| PasswordEntry::from_str(s))
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
}
