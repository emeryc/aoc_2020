use eyre::{ContextCompat, Error};
use itertools::Itertools;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
    str::FromStr,
};

type GeneratorType = ImageSection;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone)]
enum Translate {
    FlipX,
    FlipY,
    Rotate,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Vec2 { x, y }
    }
}

impl Vec2 {
    fn translate(self, translation: &[Translate], size: &Vec2) -> Self {
        translation.iter().fold(self, |point, translation| {
            let Vec2 { x, y } = point;
            let Vec2 {
                x: mut sx,
                y: mut sy,
            } = size;
            sx -= 1;
            sy -= 1;
            match translation {
                Translate::FlipX => Self { x: sx - x, y },
                Translate::FlipY => Self { x, y: sy - y },
                Translate::Rotate => Self { x: y, y: sx - x },
            }
        })
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Tile {
    Empty,
    Wave,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

use std::fmt;

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wave => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone)]
struct Map<T> {
    size: Vec2,
    tiles: Vec<T>,
}

impl<T> fmt::Debug for Map<T>
where
    T: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.get(Vec2 { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Map<T>
where
    T: Default,
{
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }
}

impl<T> Map<T>
where
    T: fmt::Debug + Clone,
{
    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vec2, tile: T) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile;
        }
    }

    fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|pos| self.tiles[pos].clone())
    }

    fn get_mut(&mut self, pos: Vec2) -> Option<&mut T> {
        self.index(pos).and_then(move |pos| self.tiles.get_mut(pos))
    }
}

#[derive(Clone)]
struct ImageSection {
    id: u64,
    image: Map<Tile>,
    translations: Vec<Translate>,
}

impl fmt::Debug for ImageSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        for y in 0..self.image.size.y {
            for x in 0..self.image.size.x {
                write!(f, "{:?}", self.get_with_border((x, y).into()).unwrap())?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Applied: {:?}", self.translations)
    }
}

impl Default for ImageSection {
    fn default() -> Self {
        Self {
            id: 0,
            image: Map::new((0, 0).into()),
            translations: Vec::new(),
        }
    }
}

macro_rules! reverse {
    ($l:expr) => {{
        let mut l = $l;
        l.reverse();
        l
    }};
}

impl ImageSection {
    fn get_with_border(&self, pos: Vec2) -> Option<Tile> {
        let pos = pos.translate(&self.translations, &self.image.size);
        self.image.get(pos)
    }
    fn left(&self) -> Vec<Tile> {
        (0..self.image.size.y)
            .map(|y| self.get_with_border(Vec2 { x: 0, y }).unwrap())
            .collect_vec()
    }
    fn right(&self) -> Vec<Tile> {
        (0..self.image.size.y)
            .map(|y| {
                self.get_with_border((self.image.size.x - 1, y).into())
                    .unwrap()
            })
            .collect_vec()
    }
    fn top(&self) -> Vec<Tile> {
        (0..self.image.size.x)
            .map(|x| self.get_with_border((x, 0).into()).unwrap())
            .collect_vec()
    }
    fn bottom(&self) -> Vec<Tile> {
        (0..self.image.size.x)
            .map(|x| {
                self.get_with_border((x, self.image.size.y - 1).into())
                    .unwrap()
            })
            .collect_vec()
    }
    fn rotate(&mut self) {
        self.translations.push(Translate::Rotate);
    }
    fn get(&self, pos: Vec2) -> Option<Tile> {
        let pos = (pos + (1, 1).into()).translate(&self.translations, &self.image.size);
        if (1..(self.image.size.x - 1)).contains(&pos.x)
            && (1..(self.image.size.y - 1)).contains(&pos.y)
        {
            self.image.get(pos)
        } else {
            None
        }
    }

    fn sides_flips(&self) -> Vec<Vec<Tile>> {
        vec![
            self.left(),
            self.right(),
            self.top(),
            self.bottom(),
            reverse!(self.left()),
            reverse!(self.right()),
            reverse!(self.top()),
            reverse!(self.bottom()),
        ]
    }
}

impl Map<ImageSection> {
    fn compress(self) -> Map<Tile> {
        let x = self.size.x * (self.tiles[0].image.size.x - 2);
        let y = self.size.y * (self.tiles[0].image.size.y - 2);
        let size = (x, y).into();
        let mut map = Map::new(size);
        for y in 0..self.size.y {
            for iy in 0..8 {
                for x in 0..self.size.x {
                    let section = self.get((x, y).into()).unwrap();
                    for ix in 0..8 {
                        map.set(
                            Vec2 {
                                x: (x * 8 + ix),
                                y: (y * 8 + iy),
                            },
                            section.get(Vec2 { x: ix, y: iy }).unwrap(),
                        );
                    }
                }
            }
        }
        map
    }
}

impl fmt::Debug for Map<ImageSection> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for iy in 0..8 {
                for x in 0..self.size.x {
                    let section = self.get((x, y).into()).unwrap();
                    for ix in 0..8 {
                        write!(f, "{:?}", section.get((ix, iy).into()).unwrap())?;
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl FromStr for ImageSection {
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
        let mut image: Map<Tile> = Map::new((10, 10).into());
        for (x, line) in input.enumerate() {
            for (y, p) in line.bytes().enumerate() {
                if p == b'#' {
                    image.set((x as _, y as _).into(), Tile::Wave);
                }
            }
        }

        Ok(ImageSection {
            id,
            image,
            translations: Default::default(),
        })
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split("\n\n")
        .map(|tile| tile.parse::<ImageSection>().unwrap())
        .collect()
}

#[aoc(day20, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    let mut map = HashMap::new();
    for tile in input {
        for s in tile.sides_flips() {
            map.entry(s).or_insert_with(Vec::new).push(tile.id);
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

    tile_edges
        .into_iter()
        .filter_map(|(k, v)| if v == 4 { Some(k) } else { None })
        .product()
}

#[aoc(day20, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    let mut edge_mapping = HashMap::new();
    for tile in input {
        for s in tile.sides_flips() {
            edge_mapping.entry(s).or_insert_with(Vec::new).push(tile.id);
        }
    }

    let mut tile_edges: HashMap<u64, usize> = HashMap::new();
    for (_, tiles) in edge_mapping.iter() {
        if tiles.len() == 1 {
            continue;
        }
        for tile in tiles.iter() {
            let edge_count = tile_edges.entry(*tile).or_insert(0);
            *edge_count += 1;
        }
    }

    let upper_right = tile_edges
        .into_iter()
        .filter_map(|(k, v)| if v == 4 { Some(k) } else { None })
        .last()
        .unwrap();

    let mut upper_right = input
        .iter()
        .find(|tile| tile.id == upper_right)
        .unwrap()
        .clone();
    while edge_mapping[&upper_right.right()].len() == 1
        || edge_mapping[&upper_right.bottom()].len() == 1
    {
        upper_right.rotate();
    }

    let mut i = 0;
    while i * i != input.len() {
        i += 1;
    }

    let mut map = Map::new((i as _, i as _).into());
    map.set((0, 0).into(), upper_right);
    for x in 1..map.size.x {
        let left = map.get_mut((x - 1, 0).into()).unwrap();
        let right_hash = left.right();
        let possible = edge_mapping.get(&right_hash).unwrap();
        assert!(possible.len() > 1);
        let right_id = *possible.iter().filter(|x| x != &&left.id).last().unwrap();
        let mut right = input.iter().find(|t| t.id == right_id).unwrap().clone();
        loop {
            if left.right() == right.left() {
                break;
            } else if left.right() == reverse!(right.left()) {
                right.translations.push(Translate::FlipX);
                if left.right() == right.left() {
                    break;
                }
            }
            right.rotate();
        }
        map.set((x, 0).into(), dbg!(right));
    }
    for y in 1..map.size.y {
        for x in 0..map.size.x {
            let above = map.get_mut((x, y - 1).into()).unwrap();
            let bottom_hash = above.bottom();
            let possible = edge_mapping.get(&bottom_hash).unwrap();
            assert!(possible.len() > 1);
            let bottom_id = *possible.iter().filter(|x| x != &&above.id).last().unwrap();
            let mut bottom = input.iter().find(|t| t.id == bottom_id).unwrap().clone();
            loop {
                let top_hash = bottom.top();
                if bottom_hash == top_hash {
                    break;
                } else if bottom_hash == reverse!(top_hash) {
                    bottom.translations.extend([Translate::FlipX].iter());
                    if above.bottom() == bottom.top() {
                        break;
                    }
                }
                bottom.rotate();
            }
            map.set((x, y).into(), bottom);
        }
    }

    let mut sea_monster: Vec<Vec2> = vec![
        (0, 1),
        (1, 0),
        (4, 0),
        (5, 1),
        (6, 1),
        (7, 0),
        (10, 0),
        (11, 1),
        (12, 1),
        (13, 0),
        (16, 0),
        (17, 1),
        (18, 2),
        (18, 1),
        (19, 1),
    ]
    .into_iter()
    .map_into()
    .collect_vec();
    let mut sm_size = Vec2 { x: 20, y: 3 };

    println!("{:?}", map);
    let map = map.compress();
    println!("{:?}", map);

    let mut cnt = 0;
    let mut tries = 0;
    loop {
        for x in 0..(map.size.x - sm_size.x) {
            for y in 0..(map.size.y - sm_size.y) {
                if sea_monster
                    .iter()
                    .all(|elem| map.get(*elem + Vec2 { x, y }).unwrap() == Tile::Wave)
                {
                    cnt += 1;
                }
            }
        }
        if cnt > 0 {
            break;
        }
        sea_monster = sea_monster
            .into_iter()
            .map(|v| v.translate(&[Translate::Rotate], &sm_size))
            .collect_vec();
        sm_size = Vec2 {
            x: sm_size.y,
            y: sm_size.x,
        };
        if tries == 4 {
            sea_monster = sea_monster
                .into_iter()
                .map(|v| v.translate(&[Translate::FlipY], &sm_size))
                .collect_vec();
        } else if tries == 8 {
            break;
        }
        tries += 1;
    }

    map.tiles.iter().filter(|t| t == &&Tile::Wave).count() as u64 - (cnt * sea_monster.len() as u64)
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
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 273);
    }

    #[test]
    fn test_translation() {
        assert_eq!(
            Vec2 { x: 0, y: 0 }.translate(&[Translate::Rotate], &Vec2 { x: 2, y: 2 }),
            Vec2 { x: 0, y: 1 }
        );
        assert_eq!(
            Vec2 { x: 0, y: 0 }.translate(
                &[Translate::Rotate, Translate::Rotate],
                &Vec2 { x: 2, y: 2 }
            ),
            Vec2 { x: 1, y: 1 }
        );
        assert_eq!(
            Vec2 { x: 0, y: 0 }.translate(&[Translate::FlipX], &Vec2 { x: 2, y: 2 }),
            Vec2 { x: 1, y: 0 }
        );
        assert_eq!(
            Vec2 { x: 0, y: 0 }.translate(&[Translate::FlipY], &Vec2 { x: 2, y: 2 }),
            Vec2 { x: 0, y: 1 }
        );
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
