use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let mut input = input.to_owned();
    input.sort_unstable();
    find_pair(input.as_slice(), 2020)
        .expect("Expect a pair")
        .iter()
        .product()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[i32]) -> i32 {
    let mut input = input.to_owned();
    input.sort_unstable();
    find_tripple(input.as_slice())
        .expect("Expect a tripple")
        .iter()
        .product()
}

fn find_pair(list: &[i32], target: i32) -> Option<[i32; 2]> {
    let mut head = 0;
    let mut tail = list.len() - 1;
    loop {
        if head >= tail {
            return None;
        }
        let sum = list.get(head).expect("Should work") + list.get(tail).expect("should work");
        match sum {
            sum if sum > target => {
                tail -= 1;
            }
            sum if sum < target => {
                tail += 1;
                head += 1;
            }
            _ => {
                break Some([
                    *list.get(head).expect("already got this"),
                    *list.get(tail).expect("already got this"),
                ]);
            }
        }
    }
}

fn find_tripple(list: &[i32]) -> Option<[i32; 3]> {
    for i in 0..(list.len() - 2) {
        if let Some(p) = find_pair(&list[i..], 2020 - list.get(i)?) {
            return Some([*list.get(i)?, p[0], p[1]]);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Result};

    use super::*;

    #[test]
    fn basic() -> Result<()> {
        let mut tdata = vec![1721, 979, 366, 299, 675, 1456];
        tdata.sort_unstable();
        let mut pair = find_pair(tdata.as_slice(), 2020).ok_or_else(|| anyhow!("No Pair"))?;
        pair.sort_unstable();
        assert_eq!(pair[0], 299);
        assert_eq!(pair[1], 1721);
        Ok(())
    }

    #[test]
    fn tripple() -> Result<()> {
        let mut tdata = vec![1721, 979, 366, 299, 675, 1456];
        tdata.sort_unstable();
        let tripple = find_tripple(tdata.as_slice()).ok_or_else(|| anyhow!("No Tripple"))?;
        let mult: i32 = Vec::from(tripple).iter().product();
        assert_eq!(mult, 241861950);
        Ok(())
    }
}
