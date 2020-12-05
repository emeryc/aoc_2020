use anyhow::Result;

#[aoc_generator(day4)]
fn generator(input: &str) -> Result<Vec<Passport>> {
    parser::passports(input)
}

#[aoc(day4, part1)]
fn solve_part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

#[aoc(day4, part2)]
fn solve_part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_2()).count()
}

#[derive(Debug, Default)]
pub struct Passport {
    expiration: Option<u32>,
    birth: Option<u32>,
    issue: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.eye_color.is_some()
            && self.birth.is_some()
            && self.issue.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.pid.is_some()
            && self.expiration.is_some()
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.
    //
    // Thinking about this, it feels like the parser should do the validation, but then we would
    // also want to not return any invalid passports, and we would have to skip them...and that
    // just seems like a lot of work...so instead this hack will have to do.
    pub fn is_valid_2(&self) -> bool {
        matches!(self.birth, Some(birth) if (1920..=2002).contains(&birth))
            && matches!(self.issue, Some(issue) if (2010..=2020).contains(&issue))
            && matches!(self.expiration, Some(expr) if (2020..=2030).contains(&expr))
            && match self.height {
                Some(ref height) => {
                    //let height = height.bytes();
                    let (val, unit) = height.split_at(height.len() - 2);

                    if unit == "in" {
                        val.parse::<u32>()
                            .map_or(false, |val| val >= 59 && val <= 76)
                    } else if unit == "cm" {
                        val.parse::<u32>()
                            .map_or(false, |val| val >= 150 && val <= 193)
                    } else {
                        false
                    }
                }
                _ => false,
            }
            && match self.hair_color {
                Some(ref color) => {
                    color.as_bytes()[0] == b"#"[0]
                        && color.bytes().skip(1).all(|c| c.is_ascii_hexdigit())
                }
                _ => false,
            }
            && matches!(self.eye_color,
                Some(ref color) if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color.as_str()))
            && matches!(self.pid, Some(ref pid) if pid.len() == 9 && pid.bytes().all(|c| c.is_ascii_digit()))
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::char,
        character::complete::none_of,
        combinator::recognize,
        multi::{many0, many1},
        sequence::{pair, terminated},
        IResult,
    };

    use anyhow::Result;

    use super::Passport;

    fn field(input: &str) -> IResult<&str, (&str, &str)> {
        pair(
            terminated(
                recognize(alt((
                    tag("byr"),
                    tag("iyr"),
                    tag("eyr"),
                    tag("hgt"),
                    tag("hcl"),
                    tag("ecl"),
                    tag("pid"),
                    tag("cid"),
                ))),
                char(':'),
            ),
            terminated(recognize(many1(none_of("\n "))), alt((tag("\n"), tag(" ")))),
        )(input)
    }

    fn passport(input: &str) -> IResult<&str, Passport> {
        let (remain, fields) = terminated(many1(field), many0(tag("\n")))(input)?;
        let mut passport = Passport::default();

        for field in fields.iter() {
            match field {
                ("byr", val) => passport.birth = Some(val.parse().expect("Should be a number")),
                ("iyr", val) => passport.issue = Some(val.parse().expect("Should be a number")),
                ("eyr", val) => {
                    passport.expiration = Some(val.parse().expect("Should be a number"))
                }
                ("hgt", val) => passport.height = Some(val.to_string()),
                ("hcl", val) => passport.hair_color = Some(val.to_string()),
                ("ecl", val) => passport.eye_color = Some(val.to_string()),
                ("pid", val) => passport.pid = Some(val.to_string()),
                ("cid", val) => passport.cid = Some(val.to_string()),
                e => unreachable!("{:?}", e),
            };
        }

        Ok((remain, passport))
    }

    pub fn passports(input: &str) -> Result<Vec<Passport>> {
        let (_, passports) = many1(passport)(input).expect("Should be fine!");

        Ok(passports)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(
            parser::passports(input)?
                .iter()
                .filter(|p| p.is_valid())
                .count(),
            2
        );

        Ok(())
    }
}
