use eyre::{Error, Result, WrapErr};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type GeneratorType = PuzzleInput;

struct Ticket {
    field_values: Vec<u64>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            field_values: s
                .split(',')
                .map(|s| s.parse::<u64>().wrap_err("Parse Failure"))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Ticket {
    fn invalid_fields<'a>(&'a self, fields: &'a [Field]) -> impl Iterator<Item = &'a u64> {
        self.field_values
            .iter()
            .filter(move |v| !fields.iter().any(|f| f.valid(v)))
    }

    fn is_valid(&self, fields: &[Field]) -> bool {
        self.field_values
            .iter()
            .all(|v| fields.iter().any(|f| f.valid(v)))
    }
}

struct Field {
    r1: (u64, u64),
    r2: (u64, u64),
    name: String,
}

impl Field {
    fn valid(&self, v: &u64) -> bool {
        (self.r1.0..=self.r1.1).contains(v) || (self.r2.0..=self.r2.1).contains(v)
    }
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let name = split.next().unwrap();
        let s = split.next().unwrap().trim();
        let split = s.split(" or ");
        let mut ranges = split.map(|range| {
            let mut range = range.split('-');
            let start = range.next().unwrap();
            let end = range.next().unwrap();

            (start.parse().unwrap(), end.parse().unwrap())
        });
        let r1 = ranges.next().unwrap();
        let r2 = ranges.next().unwrap();

        Ok(Field {
            name: name.to_string(),
            r1,
            r2,
        })
    }
}

struct PuzzleInput {
    fields: Vec<Field>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

impl PuzzleInput {
    fn invalid_error_rate(&self) -> u64 {
        self.tickets
            .iter()
            .flat_map(|ticket| ticket.invalid_fields(self.fields.as_slice()))
            .sum()
    }

    fn valid_tickets(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|t| t.is_valid(self.fields.as_slice()))
            .collect()
    }
}

impl FromStr for PuzzleInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");
        let fields: Vec<_> = sections
            .next()
            .unwrap()
            .split('\n')
            .map(|s| s.parse::<Field>())
            .collect::<Result<Vec<_>>>()?;
        let my_ticket: Ticket = sections
            .next()
            .unwrap()
            .split('\n')
            .last()
            .map(|s| s.parse())
            .unwrap()?;
        let tickets: Vec<_> = sections
            .next()
            .unwrap()
            .split('\n')
            .skip(1)
            .map(|s| s.parse::<Ticket>())
            .collect::<Result<_>>()?;
        Ok(PuzzleInput {
            fields,
            my_ticket,
            tickets,
        })
    }
}

#[aoc_generator(day16)]
fn generator(input: &str) -> Result<GeneratorType> {
    input.parse::<PuzzleInput>()
}

#[aoc(day16, part1)]
fn solve_part1(input: &GeneratorType) -> u64 {
    input.invalid_error_rate()
}

#[aoc(day16, part2)]
fn solve_part2(input: &GeneratorType) -> u64 {
    let tickets = input.valid_tickets();
    let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
    for f in input.fields.iter() {
        for t in tickets.iter() {
            let mut ticket_set = HashSet::new();
            for (i, v) in t.field_values.iter().enumerate() {
                if f.valid(&v) {
                    ticket_set.insert(i);
                };
            }
            let m2 = map
                .entry(f.name.clone())
                .or_insert_with(|| ticket_set.clone());
            *m2 = ticket_set.intersection(m2).cloned().collect();
        }
    }

    let mut done: HashMap<String, usize> = HashMap::new();

    while !map.is_empty() {
        let (r_k, r_v) = if let Some((r_k, r_v)) = map.iter().find(|(_, v)| v.len() == 1) {
            let r_v = r_v.iter().last().unwrap();
            (r_k.clone(), *r_v)
        } else {
            panic!("WTF");
        };

        map.remove(r_k.as_str());
        for (_, v) in map.iter_mut() {
            v.remove(&r_v);
        }
        done.insert(r_k, r_v);
    }

    done.iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| input.my_ticket.field_values.get(*v).unwrap())
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&generator(SAMPLE).unwrap()), 71);
    }
    //#[test]
    fn test_part2() {
        assert_eq!(solve_part2(&generator(SAMPLE).unwrap()), 0);
    }
}
