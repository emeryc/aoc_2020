use eyre::{eyre, Result, WrapErr};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type GeneratorType = Game;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Game {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

#[aoc_generator(day22)]
fn generator(input: &str) -> Result<GeneratorType> {
    let (p1, p2) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or_else(|| eyre!("Expect two sections"))?;

    let player1 = p1
        .split('\n')
        .skip(1)
        .map(|v| v.parse().wrap_err("Each line should be an int"))
        .collect::<Result<VecDeque<usize>>>()?;
    let player2 = p2
        .split('\n')
        .skip(1)
        .map(|v| v.parse().wrap_err("Each line should be an int"))
        .collect::<Result<VecDeque<usize>>>()?;

    Ok(Game { player1, player2 })
}

#[aoc(day22, part1)]
fn solve_part1(input: &GeneratorType) -> Result<usize> {
    let mut game = input.clone();
    while !(game.player1.is_empty() || game.player2.is_empty()) {
        let (p1, p2) = (
            game.player1.pop_front().unwrap(),
            game.player2.pop_front().unwrap(),
        );
        if p1 > p2 {
            game.player1.push_back(p1);
            game.player1.push_back(p2);
        } else {
            game.player2.push_back(p2);
            game.player2.push_back(p1);
        }
    }

    Ok(game
        .player1
        .iter()
        .chain(game.player2.iter())
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * (*v as usize))
        .sum())
}

#[aoc(day22, part2)]
fn solve_part2(input: &GeneratorType) -> Result<usize> {
    let mut game = input.clone();
    play_recursive(&mut game);

    Ok(game
        .player1
        .iter()
        .chain(game.player2.iter())
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * (*v as usize))
        .sum())
}

enum Player {
    P1,
    P2,
}

fn play_recursive(game: &mut GeneratorType) -> Player {
    let mut games: HashSet<Game> = HashSet::new();
    while !(game.player1.is_empty() || game.player2.is_empty()) {
        if games.contains(game) {
            return Player::P1;
        }
        games.insert(game.clone());
        let (p1, p2) = (
            game.player1.pop_front().unwrap(),
            game.player2.pop_front().unwrap(),
        );

        let winner = if game.player1.len() >= p1 && game.player2.len() >= p2 {
            play_recursive(&mut Game {
                player1: game.player1.iter().take(p1).cloned().collect(),
                player2: game.player2.iter().take(p2).cloned().collect(),
            })
        } else if p1 > p2 {
            Player::P1
        } else {
            Player::P2
        };

        match winner {
            Player::P1 => {
                game.player1.push_back(p1);
                game.player1.push_back(p2);
            }
            Player::P2 => {
                game.player2.push_back(p2);
                game.player2.push_back(p1);
            }
        }
    }

    if game.player1.is_empty() {
        Player::P2
    } else {
        Player::P1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(solve_part1(&generator(SAMPLE)?)?, 306);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(solve_part2(&generator(SAMPLE)?)?, 291);
        Ok(())
    }
    #[test]
    fn not_forever() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "Player 1:
43
19

Player 2:
2
29
14"
            )?)?,
            369
        );
        Ok(())
    }
}
