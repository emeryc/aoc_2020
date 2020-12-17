use eyre::Result;
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
    let mut n: HashSet<Point> = perms(&[*x, *y, *z])
        .into_iter()
        .map(|vec| {
            let mut ivec = vec.into_iter();
            (
                ivec.next().unwrap(),
                ivec.next().unwrap(),
                ivec.next().unwrap(),
            )
        })
        .collect();

    n.remove(cell);

    n
}

fn neigbors4(cell: &Point4) -> HashSet<Point4> {
    let (x, y, z, w) = cell;
    let mut n: HashSet<Point4> = perms(&[*x, *y, *z, *w])
        .into_iter()
        .map(|vec| {
            let mut ivec = vec.into_iter();
            (
                ivec.next().unwrap(),
                ivec.next().unwrap(),
                ivec.next().unwrap(),
                ivec.next().unwrap(),
            )
        })
        .collect();

    n.remove(cell);

    n
}

fn perms(tail: &[isize]) -> Vec<Vec<isize>> {
    if tail.is_empty() {
        vec![Vec::new()]
    } else {
        let head = tail[0];
        let p = perms(&tail[1..]);
        let mut ret = Vec::new();
        for d in -1..=1 {
            for p_v in p.iter() {
                let mut t1 = vec![head + d];
                t1.append(&mut p_v.clone());
                ret.push(t1)
            }
        }
        ret
    }
}

#[aoc(day17, part1)]
fn solve_part1(input: &GeneratorType) -> usize {
    let mut last = input.clone();
    for _ in 0..6 {
        // println!("{:?}", last);
        let mut consider: HashSet<Point> = HashSet::new();
        for x in last.iter() {
            consider.extend(neigbors(x).iter());
        }

        let mut next: HashSet<Point> = HashSet::new();
        for possible in consider.iter() {
            let n = neigbors(&possible);
            //println!("neighbors - {:?}", n);
            let on_neighbors = last.intersection(&n).count();
            if (last.contains(&possible) && (on_neighbors == 2 || on_neighbors == 3))
                || (!last.contains(&possible) && on_neighbors == 3)
            {
                next.insert(*possible);
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
        // println!("{:?}", last);
        let mut consider: HashSet<Point4> = HashSet::new();
        for x in last.iter() {
            consider.extend(neigbors4(x).iter());
        }

        let mut next: HashSet<Point4> = HashSet::new();
        for possible in consider.iter() {
            let n = neigbors4(&possible);
            //println!("neighbors - {:?}", n);
            let on_neighbors = last.intersection(&n).count();
            if (last.contains(&possible) && (on_neighbors == 2 || on_neighbors == 3))
                || (!last.contains(&possible) && on_neighbors == 3)
            {
                next.insert(*possible);
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
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&generator(SAMPLE).unwrap()), 848);
    }
}
