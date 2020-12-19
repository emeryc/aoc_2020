use eyre::Result;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

type Point = (isize, isize, isize);
type Point4 = (isize, isize, isize, isize);

type GeneratorType = HashSet<Point>;

#[aoc_generator(day17)]
fn generator(input: &str) -> Result<GeneratorType> {
    Ok(input
        .split('\n')
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, val)| {
                if val == '#' {
                    Some((x as isize, y as isize, 0))
                } else {
                    None
                }
            })
        })
        .collect())
}

fn neigbors(cell: &Point) -> HashSet<Point> {
    let (x, y, z) = cell;
    iproduct!(-1..=1, -1..=1, -1..=1)
        .map(|(d_x, d_y, d_z)| (x + d_x, y + d_y, z + d_z))
        .filter(|n| n != cell)
        .collect()
}

fn neigbors4(cell: &Point4) -> HashSet<Point4> {
    let (x, y, z, w) = cell;
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .map(|(d_x, d_y, d_z, d_w)| (x + d_x, y + d_y, z + d_z, w + d_w))
        .filter(|n| n != cell)
        .collect()
}

#[aoc(day17, part1)]
fn solve_part1(input: &GeneratorType) -> usize {
    let mut last = input.clone();
    for _ in 0..6 {
        let mut consider: HashMap<Point, usize> = HashMap::new();
        for x in last.iter().flat_map(|x| neigbors(x)) {
            let point_count = consider.entry(x).or_insert(0);
            *point_count += 1;
        }

        let mut next: HashSet<Point> = HashSet::new();
        for (possible, on_neighbors) in consider.into_iter() {
            if (last.contains(&possible) && (on_neighbors == 2 || on_neighbors == 3))
                || (!last.contains(&possible) && on_neighbors == 3)
            {
                next.insert(possible);
            };
        }
        last = next;
    }

    last.iter().count()
}

#[aoc(day17, part2)]
fn solve_part2(input: &GeneratorType) -> usize {
    let mut last = input
        .iter()
        .map(|p| {
            let (x, y, z) = p;
            (*x, *y, *z, 0)
        })
        .collect::<HashSet<Point4>>();
    for _ in 0..6 {
        let mut consider: HashMap<Point4, usize> = HashMap::new();
        for x in last.iter().flat_map(|x| neigbors4(x)) {
            let point_count = consider.entry(x).or_insert(0);
            *point_count += 1;
        }

        let mut next: HashSet<Point4> = HashSet::new();
        for (possible, on_neighbors) in consider.into_iter() {
            if (last.contains(&possible) && (on_neighbors == 2 || on_neighbors == 3))
                || (!last.contains(&possible) && on_neighbors == 3)
            {
                next.insert(possible);
            };
        }
        last = next;
    }

    last.iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = ".#.
..#
###";

    #[test]
    fn test_neighbors() {
        let mut s = HashSet::new();
        s.insert((0, 0, 0));
        assert_eq!(neigbors(&(0, 0, 0)).iter().count(), 26);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&generator(SAMPLE).unwrap()), 112);
    }
    //    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&generator(SAMPLE).unwrap()), 848);
    }
}
