use std::str::FromStr;
use std::{collections::LinkedList, fmt::Debug};

use eyre::{eyre, Error, Result};
use itertools::Itertools;

struct CupGameP1 {
    cups: LinkedList<u64>,
    current: usize,
}

impl Debug for CupGameP1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("cups:")?;
        for (i, cup) in self.cups.iter().enumerate() {
            if self.current == i {
                f.write_str(format!(" ({})", cup).as_str())?;
            } else {
                f.write_str(format!(" {}", cup).as_str())?;
            }
        }
        Ok(())
    }
}

impl FromStr for CupGameP1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups = s
            .bytes()
            .map(|b| (b - b'0') as u64)
            .collect::<LinkedList<_>>();
        Ok(CupGameP1 { cups, current: 0 })
    }
}

impl CupGameP1 {
    fn turn(&mut self) {
        let mut removed = self.cups.split_off(self.current + 1);
        let current_value = *self.cups.back().unwrap();
        let mut destination_value = current_value - 1;
        while removed.len() < 3 {
            removed.push_back(self.cups.pop_front().unwrap());
        }

        if removed.len() > 3 {
            self.cups.append(&mut removed.split_off(3));
        }

        loop {
            if destination_value == 0 {
                destination_value = *self.cups.iter().max().unwrap();
            }
            if !removed.contains(&destination_value) {
                break;
            }
            destination_value -= 1;
        }
        let destination = self
            .cups
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                if v == &destination_value {
                    Some(i)
                } else {
                    None
                }
            })
            .ok_or_else(|| eyre!("Looking for: {}", destination_value))
            .unwrap();

        let mut tail = self.cups.split_off(destination + 1);
        self.cups.append(&mut removed);
        self.cups.append(&mut tail);

        let current = self
            .cups
            .iter()
            .enumerate()
            .find_map(|(i, v)| if v == &current_value { Some(i) } else { None })
            .unwrap();
        self.current = (current + 1) % self.cups.len();
    }

    fn output(&self) -> String {
        self.cups
            .iter()
            .cycle()
            .skip_while(|v| v != &&1)
            .skip(1)
            .take(self.cups.len() - 1)
            .join("")
    }

    fn part2(&self) -> u64 {
        self.cups.iter().skip_while(|v| v != &&1).take(2).product()
    }
}

struct CupGameP2 {
    positions: Vec<usize>,
    current: usize,
    tail: usize,
}

impl FromStr for CupGameP2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = (0..s.len() + 1).collect_vec();
        let mut first = None;
        let mut last = 0;
        for (i, v) in s.bytes().map(|b| (b - b'0') as usize).tuple_windows() {
            positions[i] = v;
            if first.is_none() {
                first = Some(i)
            }
            last = v;
        }

        positions[last] = first.unwrap();

        Ok(CupGameP2 {
            positions,
            current: first.unwrap(),
            tail: last,
        })
    }
}

impl Debug for CupGameP2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} ", self.current).as_str())?;
        let mut next = self.positions[self.current];
        while next != self.current {
            f.write_str(format!("{} ", next).as_str())?;
            next = self.positions[next];
        }
        f.write_str(format!("{} ", next).as_str())?;
        Ok(())
    }
}

impl CupGameP2 {
    fn extend(&mut self) {
        self.positions[self.tail] = 10;
        for i in 10..1_000_000 {
            self.positions.push(i + 1);
        }
        self.positions.push(self.current);
    }
    fn turn(&mut self) {
        let mut destination = self.current - 1;

        let rem1 = self.positions[self.current];
        let rem2 = self.positions[rem1];
        let rem3 = self.positions[rem2];
        let heal = self.positions[rem3];

        loop {
            if destination == 0 {
                destination = self.positions.len() - 1;
            }
            if !(destination == rem1 || destination == rem2 || destination == rem3) {
                break;
            }
            destination -= 1;
        }

        // Remove 3
        self.positions[self.current] = heal;

        let heal = self.positions[destination];
        self.positions[destination] = rem1;
        self.positions[rem3] = heal;
        self.current = self.positions[self.current];
    }

    fn output(&self) -> String {
        let mut next = self.positions[1];
        let mut output = String::new();
        while next != 1 {
            output = format!("{}{}", output, next);
            next = self.positions[next];
        }
        output
    }

    fn part2(&self) -> u64 {
        let a = self.positions[1];
        let b = self.positions[a];

        println!("{:?}", &self.positions[1..100]);

        (a as u64) * (b as u64)
    }
}

#[aoc(day23, part1)]
fn solve_part1(input: &str) -> String {
    println!("???");
    let mut game = input.parse::<CupGameP2>().unwrap();
    println!("???");
    for _ in 0..100 {
        println!("{:?}", game);
        game.turn()
    }
    println!("{:?}", game);
    game.output()
}

// TOO SLOW. This uses linked lists, and should be "fast enough"
// but it seems like the built in linked lists for rust are trash
// and so this just dies on the vine, taking several seconds per
// 10_000...while waiting for this to complete, I looked around,
// others said "Linked List" and then I found a much nicer solution
// that I implemented on solve_part2.
//#[aoc(day23, part2, t2)]
fn solve_part2_take2(input: &str) -> u64 {
    let mut game = input.parse::<CupGameP1>().unwrap();
    game.cups.extend(10..=1_000_000);
    for i in 0..10_000_000 {
        if i % 10_000 == 0 {
            println!("{}", i);
        }
        game.turn()
    }
    game.part2()
}

#[aoc(day23, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut game = input.parse::<CupGameP2>().unwrap();
    game.extend();
    for _ in 0..10_000_000 {
        game.turn()
    }
    game.part2()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "389125467";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(SAMPLE), "67384529".to_string());
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(SAMPLE), 149245887792);
    }
}
