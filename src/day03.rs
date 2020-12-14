use eyre::{Error, Result};
use std::collections::HashSet;
use std::{cmp::max, str::FromStr};

#[derive(Debug)]
struct Map {
    width: usize,
    pub height: usize,
    trees: HashSet<[usize; 2]>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: HashSet<_> = s
            .split('\n')
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(col_idx, element)| match element {
                        '#' => Some([col_idx, row_idx]),
                        _ => None,
                    })
            })
            .collect();

        let (width, height) = trees.iter().fold((0, 0), |(max_x, max_y), [x, y]| {
            (max(*x, max_x), max(*y, max_y))
        });

        Ok(Map {
            width,
            height,
            trees,
        })
    }
}

impl Map {
    pub fn in_path(&self, path: impl Iterator<Item = [usize; 2]>) -> u128 {
        let path_set = path
            .map(|[x, y]| [x % (self.width + 1), y])
            .collect::<HashSet<_>>();
        self.trees.intersection(&path_set).count() as u128
    }
}

struct Toboggan {
    fall: usize,
    run: usize,
    height: usize,
    count: usize,
}
impl Toboggan {
    pub fn path(fall: usize, run: usize, height: usize) -> Self {
        Toboggan {
            fall,
            run,
            height,
            count: 0,
        }
    }
}

impl Iterator for Toboggan {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.run * self.count;
        let y = self.fall * self.count;
        self.count += 1;

        if y > self.height {
            None
        } else {
            Some([x, y])
        }
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Result<Map> {
    input.parse()
}

#[aoc(day3, part1)]
fn solve_part1(map: &Map) -> u128 {
    map.in_path(Toboggan::path(1, 3, map.height))
}

#[aoc(day3, part2)]
fn solve_part2(map: &Map) -> u128 {
    vec![[1 as usize, 1 as usize], [1, 3], [1, 5], [1, 7], [2, 1]]
        .iter()
        .map(|[fall, run]| map.in_path(Toboggan::path(*fall, *run, map.height)))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_1() -> Result<()> {
        let map = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            .parse::<Map>()?;

        println!("{:#?}", map);
        assert_eq!(map.in_path(Toboggan::path(1, 3, map.height)), 7);

        Ok(())
    }
}
