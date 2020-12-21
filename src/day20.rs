use eyre::{ContextCompat, Error};
use itertools::Itertools;
use std::{collections::HashMap, convert::TryInto, str::FromStr};

type GeneratorType = Tile;

struct Tile {
    id: u64,
    image: [[u8; 10]; 10],
}

impl Tile {
    fn sides(&self) -> [u16; 8] {
        let mut sides: [u16; 8] = [0; 8];
        for (i, side) in sides.iter_mut().enumerate() {
            let mut hash: u16 = 0;
            for x in 0..10 {
                let pair = if i == 0 {
                    (0, x)
                } else if i == 1 {
                    (x, 0)
                } else if i == 2 {
                    (9, x)
                } else if i == 3 {
                    (x, 9)
                } else if i == 4 {
                    (0, 9 - x)
                } else if i == 5 {
                    (9 - x, 0)
                } else if i == 6 {
                    (9, 9 - x)
                } else {
                    (9 - x, 9)
                };
                hash = hash << 1 | (self.image[pair.0][pair.1] as u16);
            }
            *side = hash;
        }
        sides
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split('\n');
        let id = input
            .next()
            .context("Tile ID")?
            .split_at(5)
            .1
            .strip_suffix(':')
            .context("???")?
            .parse()?;
        let mut image = [[0; 10]; 10];
        for (i, line) in input.enumerate() {
            let row: Vec<u8> = line
                .bytes()
                .map(|b| if b == b'#' { 1 } else { 0 })
                .collect_vec();
            let row: &[u8; 10] = row.as_slice().try_into()?;
            image[i] = *row;
        }

        Ok(Tile { id, image })
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split("\n\n")
        .map(|tile| tile.parse::<Tile>().unwrap())
        .collect()
}

#[aoc(day20, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    let mut map: HashMap<u16, Vec<u64>> = HashMap::new();
    for tile in input {
        for s in tile.sides().iter() {
            map.entry(*s).or_insert(Vec::new()).push(tile.id);
        }
    }

    let mut tile_edges: HashMap<u64, usize> = HashMap::new();
    for (_, tiles) in map.into_iter() {
        if tiles.len() == 1 {
            continue;
        }
        for tile in tiles.iter() {
            let edge_count = tile_edges.entry(*tile).or_insert(0);
            *edge_count += 1;
        }
    }

    println!("{:#?}", tile_edges);

    tile_edges
        .into_iter()
        .filter_map(|(k, v)| if v == 4 { Some(k) } else { None })
        .product()
}

#[aoc(day20, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 20899048083289);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 0);
    }

    const SAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
}
