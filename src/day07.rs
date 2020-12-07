use anyhow::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::char,
    character::complete::one_of,
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, str::FromStr};

#[aoc_generator(day7)]
fn generator(input: &str) -> Result<Vec<Rule>> {
    input.split("\n").map(|rule| rule.parse::<Rule>()).collect()
}

struct Rule {
    color: String,
    contains: Vec<(u32, String)>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (bag_color, bag_contains)) = pair(color, contains)(s).expect("Should be parsable");
        Ok(Rule {
            color: bag_color.to_string(),
            contains: bag_contains
                .iter()
                .filter(|(count, _)| count != &"no")
                .map(|(count, color)| Ok((count.parse::<u32>()?, color.to_string())))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

fn color(input: &str) -> IResult<&str, &str> {
    terminated(
        take_until(" bag"),
        tuple((tag(" bag"), opt(char('s')), opt(tag(" contain ")))),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn contains(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    terminated(
        many1(terminated(
            separated_pair(alt((decimal, tag("no"))), char(' '), color),
            opt(tag(", ")),
        )),
        char('.'),
    )(input)
}

#[aoc(day7, part1)]
fn solve_part1(input: &[Rule]) -> u32 {
    let mut holds_key: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rule in input.iter() {
        for (_, bag) in rule.contains.iter() {
            holds_key.entry(&bag).or_default().insert(&rule.color);
        }
    }

    let mut seed = holds_key
        .get("shiny gold")
        .expect("Should be a shiny gold")
        .clone();
    let mut last = 0;

    while last != seed.len() {
        last = seed.len();
        for s in seed.clone().iter() {
            if let Some(c) = holds_key.get(s) {
                seed.extend(c);
            }
        }
    }

    seed.len() as u32
}

#[derive(Debug)]
struct BagHolder<'a> {
    queue: VecDeque<(u32, &'a str)>,
    count: u32,
}

impl<'a> BagHolder<'a> {
    fn new(count: u32, first: &'a str) -> Self {
        let mut queue: VecDeque<(u32, &str)> = VecDeque::new();
        queue.push_back((count, first));
        BagHolder { count: 0, queue }
    }

    fn add(&mut self, outer: u32, count: u32, next: &'a str) {
        let boxes = outer * count;
        self.queue.push_back((boxes, next));
        self.count += boxes;
    }

    fn next(&mut self) -> Option<(u32, &'a str)> {
        self.queue.pop_front()
    }
}

#[aoc(day7, part2)]
fn solve_part2(input: &[Rule]) -> u32 {
    let map: HashMap<&str, Vec<(u32, &str)>> = input
        .iter()
        .map(|r| {
            (
                r.color.as_str(),
                r.contains
                    .iter()
                    .map(|(u, s)| (*u, s.as_str()))
                    .collect::<Vec<(u32, &str)>>(),
            )
        })
        .collect();
    let mut holder = BagHolder::new(1, "shiny gold");
    while let Some((count, color)) = holder.next() {
        if let Some(contains) = map.get(color) {
            for (i, c) in contains.iter() {
                holder.add(count, *i, c);
            }
            println!("{:?}", holder);
        }
    }

    holder.count
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_color() -> Result<()> {
        let (_, bag_color) =
            color("light red bags contain 1 bright white bag, 2 muted yellow bags.")?;

        assert_eq!(bag_color, "light red");

        Ok(())
    }
    #[test]
    fn test_contains() -> Result<()> {
        let (input, _) = color("light red bags contain 1 bright white bag, 2 muted yellow bags.")?;
        let (_, contains) = contains(input)?;

        assert_eq!(contains.get(0).unwrap(), &("1", "bright white"));

        Ok(())
    }
    #[test]
    fn sample_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve_part1(generator(input).unwrap().as_slice()), 4);
    }
    #[test]
    fn sample_part2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve_part2(generator(input).unwrap().as_slice()), 32);
    }

    #[test]
    fn sample_part2_alt() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(solve_part2(generator(input).unwrap().as_slice()), 126);
    }
}
