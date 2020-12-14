use anyhow::{Context, Result};
type GeneratorType = State;

#[derive(Debug, Clone)]
struct State {
    arrival: u64,
    ids: Vec<Option<u64>>,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Result<GeneratorType> {
    let mut input = input.split('\n');
    Ok(State {
        arrival: input
            .next()
            .with_context(|| "Split on \\n and 0 is empty")?
            .parse()?,
        ids: input
            .next()
            .with_context(|| "Split on \\n and 1 is empty")?
            .split(',')
            .map(|x| x.parse().ok())
            .collect(),
    })
}

#[aoc(day13, part1)]
fn solve_part1(input: &GeneratorType) -> Result<u64> {
    let min = input
        .ids
        .iter()
        .filter_map(|id| {
            id.map(|v| {
                let last_before = input.arrival % v;
                let wait_time = v - last_before;
                (wait_time, v)
            })
        })
        .min_by_key(|v| v.0)
        .with_context(|| "No Minimum?")?;

    Ok(min.0 * min.1)
}

#[aoc(day13, part2)]
fn solve_part2(input: &GeneratorType) -> Result<u64> {
    //let max = input
    //    .ids
    //    .iter()
    //    .enumerate()
    //    .max_by_key(|(idx, id)| id.unwrap_or(0))
    //    .with_context(|| "No root id")?;
    //let base = max.1.with_context(|| "???")?;
    //let mut i: u64 = base - max.0 as u64;
    ////println!("{} {} {} {}", max.0, base, i, (i + base + base) % base);
    //let mut failure = false;
    //loop {
    //    for (idx, id) in input.ids.iter().enumerate() {
    //        if id.is_none() {
    //            continue;
    //        }
    //        if (i + idx as u64) % id.unwrap() != 0 {
    //            //println!("{}, {}", idx, i);
    //            failure = true;
    //            break;
    //        }
    //    }
    //    if !failure {
    //        break;
    //    }
    //    failure = false;
    //    i += base;
    //}
    let mut earliest_time = 0;
    let mut running_product = 1;
    for (idx, id) in input.ids.iter().enumerate() {
        let idx = idx as u64;
        if let Some(id) = id {
            while (earliest_time + idx) % id != 0 {
                earliest_time += running_product;
            }
            running_product *= id
        }
    }

    Ok(earliest_time)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(solve_part1(&generator(SAMPLE)?)?, 295);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(solve_part2(&generator(SAMPLE)?)?, 1068781);
        Ok(())
    }
    #[test]
    fn test_part2_1() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "12
17,x,13,19"
            )?)?,
            3417
        );
        Ok(())
    }
    #[test]
    fn test_part2_2() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "12
67,7,59,61"
            )?)?,
            754018
        );
        Ok(())
    }
    #[test]
    fn test_part2_3() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "12
67,x,7,59,61"
            )?)?,
            779210
        );
        Ok(())
    }
    #[test]
    fn test_part2_4() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "12
67,7,x,59,61"
            )?)?,
            1261476
        );
        Ok(())
    }
    #[test]
    fn test_part2_5() -> Result<()> {
        assert_eq!(
            solve_part2(&generator(
                "12
1789,37,47,1889"
            )?)?,
            1202161486
        );
        Ok(())
    }
}
