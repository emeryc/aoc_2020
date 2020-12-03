use anyhow::{Error, Result};
use std::io::prelude::*;
use std::{cmp::max, fs::File, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug)]
struct Map {
    width: usize,
    pub height: usize,
    trees: Vec<[usize; 2]>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<_> = s
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
    pub fn is_tree(&self, pos: &[usize; 2]) -> bool {
        let [x, y] = *pos;
        let x = if x > self.width {
            x % (self.width + 1)
        } else {
            x
        };

        if y > self.height {
            false
        } else {
            self.trees.contains(&([x, y]))
        }
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

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut file = File::open(args.input)?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let map: Map = input.parse()?;
    let trees = Toboggan::path(1, 3, map.height)
        .filter(|pos| map.is_tree(pos))
        .count();

    println!("Trees: {}", trees);

    let product: u128 = vec![[1 as usize, 1 as usize], [1, 3], [1, 5], [1, 7], [2, 1]]
        .iter()
        .map(|[fall, run]| {
            Toboggan::path(*fall, *run, map.height)
                .filter(|pos| map.is_tree(pos))
                .count() as u128
        })
        .product();

    println!("Tree Product: {}", product);

    Ok(())
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
        assert_eq!(
            Toboggan::path(1, 3, map.height)
                .filter(|pos| map.is_tree(pos))
                .count(),
            7
        );

        Ok(())
    }
}
