use anyhow::anyhow;
use anyhow::Result;
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

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

    let i_list = input
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();

    let pair = find_pair(i_list.as_slice(), 2020).ok_or_else(|| anyhow!("No pair found"))?;

    println!("{}", pair.iter().product::<i32>());

    let tripple = find_tripple(i_list.as_slice()).ok_or_else(|| anyhow!("No tripple found"))?;

    println!("{}", tripple.iter().product::<i32>());

    Ok(())
}

fn find_pair(list: &[i32], target: i32) -> Option<[i32; 2]> {
    let mut list = Vec::from(list);
    list.sort_unstable();
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

    use crate::{find_pair, find_tripple};

    #[test]
    fn basic() -> Result<()> {
        let mut pair = find_pair(vec![1721, 979, 366, 299, 675, 1456].as_slice(), 2020)
            .ok_or_else(|| anyhow!("No Pair"))?;
        pair.sort_unstable();
        assert_eq!(pair[0], 299);
        assert_eq!(pair[1], 1721);
        Ok(())
    }

    #[test]
    fn tripple() -> Result<()> {
        let tripple = find_tripple(vec![1721, 979, 366, 299, 675, 1456].as_slice())
            .ok_or_else(|| anyhow!("No Tripple"))?;
        let mult: i32 = Vec::from(tripple).iter().product();
        assert_eq!(mult, 241861950);
        Ok(())
    }
}
