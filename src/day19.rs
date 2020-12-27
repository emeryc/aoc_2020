use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

use eyre::{ContextCompat, Error, Result};
type GeneratorType = Puzzle;

#[derive(Debug, Clone)]
struct Rule {
    id: usize,
    element: Element,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let id = split.next().context("Must have an ID")?.parse()?;
        let split = split.next().context("Must have a rule")?.split_whitespace();
        let mut rule: Option<Element> = None;
        for r in split {
            match r {
                r if r.contains('"') => {
                    rule = Some(Element::Char(r.trim_matches('"').to_string()));
                }
                "|" => {
                    rule = Some(Element::Or(
                        Box::new(rule.context("Or needs one side filled already")?),
                        Box::new(Element::Ids(Vec::new())),
                    ))
                }
                r => {
                    if let Some(Element::Ids(mut v)) = rule {
                        v.push(r.parse()?);
                        rule = Some(Element::Ids(v));
                    } else if let Some(Element::Or(e, mut b)) = rule {
                        if let Element::Ids(ref mut v) = b.as_mut() {
                            v.push(r.parse()?);
                            rule = Some(Element::Or(e, b)); //Box::new(Element::Ids(v))));
                        } else {
                            unreachable!()
                        }
                    } else {
                        rule = Some(Element::Ids(vec![r.parse()?]))
                    }
                }
            }
        }

        Ok(Rule {
            id,
            element: rule.context("Must have a rule")?,
        })
    }
}

#[derive(Debug, Clone)]
enum Element {
    Char(String),
    Ids(Vec<usize>),
    Or(Box<Element>, Box<Element>),
}

#[derive(Debug, Clone)]
struct Puzzle {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
    cnf: Vec<CNF>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CNF {
    id: usize,
    rule: CNFR,
}

impl Debug for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}: {:?}", self.id, self.rule).as_str())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum CNFR {
    Term(u8),
    Bin(usize, usize),
}
impl Debug for CNFR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CNFR::Term(u) => f.write_str(format!("\"{}\"", *u as char).as_str()),
            CNFR::Bin(a, b) => f.write_str(format!("{} {}", a, b).as_str()),
        }
    }
}

enum Inter {
    Term(u8),
    Bin(usize, usize),
    Unit(usize),
}

fn map_rule(id: usize, rule: &Element) -> Vec<(usize, Inter)> {
    match rule {
        Element::Char(c) => vec![(id, Inter::Term(c.bytes().last().unwrap()))],
        Element::Ids(v) => {
            if v.len() == 1 {
                vec![(id, Inter::Unit(*v.get(0).unwrap()))]
            } else if v.len() == 2 {
                vec![(id, Inter::Bin(*v.get(0).unwrap(), *v.get(1).unwrap()))]
            } else {
                panic!("Whee!!")
            }
        }
        Element::Or(ref a, ref b) => {
            let mut or = map_rule(id, a);
            or.append(&mut map_rule(id, b));
            or
        }
    }
}

impl Puzzle {
    fn new(rules: HashMap<usize, Rule>, messages: Vec<String>) -> Self {
        let inter = rules
            .values()
            .flat_map(|r| map_rule(r.id, &r.element))
            .collect::<Vec<_>>();
        let unit: Vec<_> = inter
            .iter()
            .filter_map(|(id, r)| {
                if let Inter::Unit(u) = r {
                    Some((id, u))
                } else {
                    None
                }
            })
            .collect();
        let mut cnf = Vec::new();
        for (id, r) in inter.iter().filter(|(_, r)| !matches!(r, Inter::Unit(_))) {
            match r {
                Inter::Term(c) => cnf.push(CNF {
                    id: *id,
                    rule: CNFR::Term(*c),
                }),
                Inter::Bin(a, b) => {
                    let mut last = vec![(*a, *b)];
                    loop {
                        let mut tmp = Vec::new();
                        for (a, b) in last.iter() {
                            if unit.iter().any(|(id, _)| *id == a) {
                                tmp.extend(
                                    &mut unit
                                        .iter()
                                        .filter(|(id, _)| a == *id)
                                        .map(|(_, r)| (**r, *b)),
                                )
                            } else if unit.iter().any(|(id, _)| *id == b) {
                                tmp.extend(
                                    &mut unit
                                        .iter()
                                        .filter(|(id, _)| b == *id)
                                        .map(|(_, r)| (*a, **r)),
                                )
                            } else {
                                tmp.push((*a, *b));
                            }
                        }
                        if last == tmp {
                            break;
                        } else {
                            last = tmp;
                        }
                    }
                    cnf.append(
                        &mut last
                            .into_iter()
                            .map(|(a, b)| CNF {
                                id: *id,
                                rule: CNFR::Bin(a, b),
                            })
                            .collect(),
                    );
                    cnf.push(CNF {
                        id: *id,
                        rule: CNFR::Bin(*a, *b),
                    });
                }
                Inter::Unit(_) => unreachable!(),
            }
        }
        Puzzle {
            cnf: cnf.iter().unique().cloned().collect(),
            rules,
            messages,
        }
    }

    fn get_patterns(&self, id: &usize, max_len: usize) -> HashSet<String> {
        let rule = self.rules.get(id).unwrap();
        self.resolve(&rule.element, max_len)
    }

    fn resolve(&self, elem: &Element, max_len: usize) -> HashSet<String> {
        let mut collection = HashSet::new();
        match elem {
            Element::Char(c) => {
                collection.insert(c.clone());
            }
            Element::Ids(v) => {
                if max_len == 0 {
                    return collection;
                }
                let next_len = max_len.saturating_sub(v.len());
                let mut i = v.iter();
                collection = self.get_patterns(i.next().unwrap(), next_len);
                for next in i {
                    collection = collection
                        .iter()
                        .cartesian_product(self.get_patterns(next, next_len).iter())
                        .map(|(a, b)| format!("{}{}", a, b))
                        .collect();
                }
            }
            Element::Or(e1, e2) => {
                collection = self
                    .resolve(e1.as_ref(), max_len)
                    .union(&self.resolve(e2.as_ref(), max_len))
                    .cloned()
                    .collect();
            }
        };
        collection
    }

    fn check(&self, input: &str) -> bool {
        let mut production: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (i, c) in input.bytes().enumerate() {
            let m = self
                .cnf
                .iter()
                .filter(|r| matches!(r.rule, CNFR::Term(p) if p == c))
                .map(|r| r.id)
                .collect_vec();
            production.insert((1, i + 1), m);
        }

        for span_len in 2..=input.len() {
            for start in 1..=(input.len() - span_len + 1) {
                for partition in 1..=span_len {
                    let ra = (partition, start);
                    let rb = (span_len - partition, start + partition);
                    if let (Some(ref b), Some(ref c)) = (production.get(&ra), production.get(&rb)) {
                        let v = b
                            .iter()
                            .cartesian_product(c.iter())
                            .map(|(b, c)| CNFR::Bin(*b, *c))
                            .collect_vec();
                        let mut v = v
                            .iter()
                            .flat_map(|b| {
                                self.cnf.iter().filter(move |r| r.rule == *b).map(|r| r.id)
                            })
                            .collect_vec();
                        production
                            .entry((span_len, start))
                            .or_insert(Vec::new())
                            .append(&mut v);
                    };
                }
            }
        }
        production
            .get(&(input.len(), 1))
            .map_or(false, |v| v.contains(&0))
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Result<GeneratorType> {
    let mut split = input.split("\n\n");
    let rules = split.next().context("Two sections needed")?;
    let messages = split.next().context("Two sections needed")?;

    Ok(Puzzle::new(
        rules
            .split('\n')
            .map(|l| {
                let rule = l.parse::<Rule>()?;
                Ok((rule.id, rule))
            })
            .collect::<Result<HashMap<usize, Rule>>>()?,
        messages.split('\n').map(|s| s.to_string()).collect(),
    ))
}

#[aoc(day19, part1)]
fn solve_part1(input: &GeneratorType) -> usize {
    input.messages.iter().filter(|m| input.check(m)).count()
}

#[aoc(day19, part2)]
fn solve_part2(input: &GeneratorType) -> usize {
    let mut input = input.clone();
    input
        .rules
        .insert(8, "8: 42 | 42 8".parse::<Rule>().unwrap());
    input
        .rules
        .insert(11, "11: 42 31 | 42 132".parse::<Rule>().unwrap());
    input
        .rules
        .insert(132, "132: 11 31".parse::<Rule>().unwrap());
    input = Puzzle::new(input.rules, input.messages);

    input.messages.iter().filter(|m| input.check(m)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"0: 4 6
6: 7
7: 1 5
8: 2 3
9: 10
10: 3 2
1: 8 | 9
2: 4 4 | 5 5
11: 4 5
12: 5 4
3: 11 | 12
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_valid_production2() {
        let puzzle = generator(SAMPLE).unwrap();
        assert!(puzzle.check("abbbab"));
    }
    #[test]
    fn test_valid_production() {
        let puzzle = generator(SAMPLE).unwrap();
        assert!(puzzle.check("ababbb"));
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&generator(SAMPLE).unwrap()), 2);
    }

    #[test]
    fn test_valid_production3() {
        let puzzle = generator(
            r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 132
132: 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

a"#,
        )
        .unwrap();
        let mut cnf = puzzle.cnf.clone();
        cnf.sort_by_key(|c| c.id);
        assert!(puzzle.check("bbabbbbaabaabba"));
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(
                &generator(
                    r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
                )
                .unwrap()
            ),
            12
        );
    }
}
