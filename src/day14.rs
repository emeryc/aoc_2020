use eyre::{eyre, Error, Result};
use std::{collections::HashMap, str::FromStr};

type GeneratorType = Instruction;

#[derive(Debug, Clone)]
struct Memory {
    registers: HashMap<usize, u64>,
    ones_mask: u64,
    zeros_mask: u64,
}

impl Memory {
    fn new() -> Self {
        Memory {
            registers: HashMap::new(),
            ones_mask: 0,
            zeros_mask: 1,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask_str) => {
                self.ones_mask = mask_str.chars().fold(0, |acc, c| match c {
                    '1' => (acc << 1) | 1,
                    _ => (acc << 1),
                });
                self.zeros_mask = mask_str.chars().fold(0, |acc, c| match c {
                    '0' => acc << 1,
                    _ => (acc << 1) | 1,
                });
            }
            Instruction::Mem { register, value } => {
                let entry = self.registers.entry(*register).or_insert(0);
                *entry = (value | self.ones_mask) & self.zeros_mask;
            }
        };
    }

    fn sum(&self) -> u64 {
        self.registers.values().sum()
    }
}

#[derive(Debug)]
struct Memory2 {
    registers: HashMap<usize, u64>,
    masks: Vec<(usize, usize)>,
}

impl Memory2 {
    fn new() -> Self {
        Memory2 {
            registers: HashMap::new(),
            masks: Vec::new(),
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask_str) => {
                self.masks = mask_str.chars().fold(vec![(0, 0)], |acc, c| match c {
                    '1' => acc
                        .iter()
                        .map(|(ones, zeros)| ((ones << 1) | 1, (zeros << 1) | 1))
                        .collect(),
                    'X' => acc
                        .iter()
                        .map(|(ones, zeros)| ((ones << 1) | 1, (zeros << 1) | 1))
                        .chain(acc.iter().map(|(ones, zeros)| (ones << 1, zeros << 1)))
                        .collect(),
                    _ => acc
                        .iter()
                        .map(|(ones, zeros)| ((ones << 1), (zeros << 1) | 1))
                        .collect(),
                });
            }
            Instruction::Mem { register, value } => {
                for (ones, zeros) in self.masks.iter() {
                    let val = self.registers.entry((register | ones) & zeros).or_insert(0);
                    *val = *value;
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.registers.values().sum()
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask(String),
    Mem { register: usize, value: u64 },
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inst = s.split('=');
        let var_name = match inst.next().ok_or_else(|| eyre!("No Initial Match"))?.trim() {
            "mask" => Instruction::Mask(
                inst.next()
                    .ok_or_else(|| eyre!("No Mask"))?
                    .trim()
                    .to_string(),
            ),
            mem => {
                let (_, register) = mem.split_at(4);
                let register = register[..register.len() - 1].parse()?;
                let value = inst
                    .next()
                    .ok_or_else(|| eyre!("No mem value"))?
                    .trim()
                    .parse()?;
                Instruction::Mem { register, value }
            }
        };
        Ok(var_name)
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

#[aoc(day14, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    let mut mem = Memory::new();
    input.iter().for_each(|i| {
        mem.apply(i);
    });
    mem.sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    let mut mem = Memory2::new();
    input.iter().for_each(|i| {
        mem.apply(i);
    });
    mem.sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 165);
    }
    #[test]
    fn test_part2() {
        let sample = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(solve_part2(generator(sample).as_slice()), 208);
    }
}
