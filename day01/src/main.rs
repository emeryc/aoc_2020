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

    let pair = find_pair(i_list.as_slice());

    println!("{}", pair.iter().product::<i32>());

    let tripple = find_tripple(i_list.as_slice());

    println!("{}", tripple.iter().product::<i32>());

    Ok(())
}

fn find_pair(list: &[i32]) -> [i32; 2] {
    let mut list = Vec::from(list);
    list.sort();
    let mut head = 0;
    let mut tail = list.len() / 2;
    loop {
        let sum = dbg!(list.get(head).expect("Should work") + list.get(tail).expect("should work"));
        match sum {
            sum if sum > 2020 => {
                tail -= 1;
            }
            sum if sum < 2020 => {
                tail += 1;
                head += 1;
            }
            _ => {
                break [
                    *list.get(head).expect("already got this"),
                    *list.get(tail).expect("already got this"),
                ];
            }
        }
    }
}

fn find_tripple(list: &[i32]) -> [i32; 3] {
    for i in 0..(list.len() - 2) {
        for j in i..(list.len() - 1) {
            for k in j..list.len() {
                if list.get(i).unwrap() + list.get(j).unwrap() + list.get(k).unwrap() == 2020 {
                    return [
                        *list.get(i).unwrap(),
                        *list.get(j).unwrap(),
                        *list.get(k).unwrap(),
                    ];
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use crate::{find_pair, find_tripple};

    #[test]
    fn basic() {
        let mut pair = find_pair(vec![1721, 979, 366, 299, 675, 1456].as_slice());
        pair.sort();
        assert_eq!(pair[0], 299);
        assert_eq!(pair[1], 1721);
    }

    #[test]
    fn tripple() {
        let tripple = dbg!(find_tripple(
            vec![1721, 979, 366, 299, 675, 1456].as_slice()
        ));
        let mult: i32 = Vec::from(tripple).iter().product();
        assert_eq!(mult, 241861950);
    }
}
