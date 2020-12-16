use std::{collections::HashMap, str::FromStr};
type GeneratorType = u64;

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split(',')
        .map(u64::from_str)
        .filter_map(Result::ok)
        .collect()
}

struct MemoryGame {
    turn: u64,
    last_spoken: HashMap<u64, u64>,
    last: u64,
}

impl MemoryGame {
    fn new(start: &[GeneratorType]) -> Self {
        MemoryGame {
            turn: start.len() as u64,
            last_spoken: start
                .iter()
                .enumerate()
                .map(|(i, v)| (*v, i as u64))
                .collect(),
            last: 0,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.last;
        if let Some(last_turn) = self.last_spoken.get(&last) {
            let next_spoke = self.turn - last_turn;
            self.last = next_spoke;
        } else {
            self.last = 0;
        };
        self.last_spoken.insert(last, self.turn);
        self.turn += 1;

        Some(self.last)
    }
}

#[aoc(day15, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    MemoryGame::new(input)
        .take(2020 - (input.len() + 1))
        .last()
        .unwrap()
}

#[aoc(day15, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    MemoryGame::new(input)
        .take(30000000 - (input.len() + 1))
        .last()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "1,3,2";

    #[test]
    fn test_sample() {
        let mut game = MemoryGame::new(&[0, 3, 6]);
        //assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(1));
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(4));
        assert_eq!(game.next(), Some(0));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 1);
    }
    //#[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 2578);
    }
}
