use nom::{
    branch::alt, bytes::complete::tag, combinator::recognize, error::Error, multi::many1, Finish,
    IResult,
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
type GeneratorType = Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    e: usize,
    se: usize,
    sw: usize,
    w: usize,
    nw: usize,
    ne: usize,
}

macro_rules! dir {
    ($s:ident, $d:ident, $set:ident) => {
        let mut c = $s.clone();
        c.$d += 1;
        c.simplify();
        $set.insert(c);
    };
}

macro_rules! shorten {
    ($tile:ident, $a:ident, $b:ident, $to:ident) => {
        if $tile.$a > 0 && $tile.$b > 0 {
            let shift = $tile.$a.min($tile.$b);
            $tile.$a -= shift;
            $tile.$b -= shift;
            $tile.$to += shift;
        }
    };
}

macro_rules! cancel {
    ($tile:ident, $a:ident, $b:ident) => {
        if $tile.$a > 0 && $tile.$b > 0 {
            let shift = $tile.$a.min($tile.$b);
            $tile.$a -= shift;
            $tile.$b -= shift;
        }
    };
}

impl Tile {
    fn simplify(&mut self) {
        let mut hash = 0;
        while hash != calculate_hash(self) {
            hash = calculate_hash(self);
            shorten!(self, nw, sw, w);
            shorten!(self, ne, se, e);
            shorten!(self, w, ne, nw);
            shorten!(self, w, se, sw);
            shorten!(self, e, nw, ne);
            shorten!(self, e, sw, se);
            cancel!(self, e, w);
            cancel!(self, ne, sw);
            cancel!(self, nw, se);
        }
    }

    fn adjacent(&self) -> HashSet<Self> {
        let mut set = HashSet::new();
        dir!(self, nw, set);
        dir!(self, ne, set);
        dir!(self, w, set);
        dir!(self, e, set);
        dir!(self, sw, set);
        dir!(self, se, set);

        set
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            e: 0,
            se: 0,
            sw: 0,
            w: 0,
            nw: 0,
            ne: 0,
        }
    }
}

fn parse_directions(s: &str) -> IResult<&str, Vec<&str>> {
    let (i, dirs) = many1(recognize(alt((
        tag("e"),
        tag("se"),
        tag("sw"),
        tag("w"),
        tag("nw"),
        tag("ne"),
    ))))(s)?;

    Ok((i, dirs))
}

impl FromStr for Tile {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_directions(s).finish() {
            Ok((_remaining, dirs)) => Ok(dirs.iter().fold(Tile::default(), |mut tile, dir| {
                match *dir {
                    "e" => tile.e += 1,
                    "se" => tile.se += 1,
                    "sw" => tile.sw += 1,
                    "w" => tile.w += 1,
                    "nw" => tile.nw += 1,
                    "ne" => tile.ne += 1,
                    _ => unreachable!("By problem statement"),
                };
                tile
            })),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|tile| tile.parse::<Tile>().unwrap())
        .collect()
}

#[aoc(day24, part1)]
fn solve_part1(input: &[GeneratorType]) -> usize {
    let mut map = HashMap::new();
    for mut tile in input.to_vec().into_iter() {
        tile.simplify();
        let count = map.entry(tile).or_insert(0);
        *count += 1;
    }

    map.iter().filter(|(_, c)| *c % 2 == 1).count()
}

#[aoc(day24, part2)]
fn solve_part2(input: &[GeneratorType]) -> usize {
    let mut map = HashMap::new();
    for mut tile in input.to_vec().into_iter() {
        tile.simplify();
        let count = map.entry(tile).or_insert(0);
        *count += 1;
    }
    let mut on = map
        .into_iter()
        .filter(|(_, c)| *c % 2 == 1)
        .map(|(t, _)| t)
        .collect::<HashSet<_>>();

    for i in 0..100 {
        let mut tiles = HashMap::new();
        for tile in on.iter() {
            for adj in tile.adjacent() {
                let count = tiles.entry(adj).or_insert(0);
                *count += 1;
            }
        }

        on = tiles
            .into_iter()
            .filter_map(|(tile, count)| {
                if count == 2 || count == 1 && on.contains(&tile) {
                    Some(tile)
                } else {
                    None
                }
            })
            .collect();
    }

    on.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 10);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 2208);
    }
}
