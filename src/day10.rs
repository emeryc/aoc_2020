use std::cell::Cell;

type GeneratorType = u64;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    let mut input = input.to_vec();
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort_unstable();
    let (ones, threes) =
        input
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold((0, 0), |(o, t), v| {
                if v == 1 {
                    (o + 1, t)
                } else if v == 3 {
                    (o, t + 1)
                } else {
                    (o, t)
                }
            });
    println!("ones: {}, threes: {}", ones, threes);
    ones * threes
}

#[aoc(day10, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    let mut input = input.to_vec();
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort_unstable();
    let mut input = input
        .iter()
        .map(|v| (*v, Cell::new(0)))
        .collect::<Vec<(u64, Cell<u64>)>>();
    input.get_mut(0).unwrap().1.set(1);

    for (index, (volt, paths)) in input.iter().enumerate() {
        for window in 1..=3 {
            if let Some((next_volt, next_paths)) = input.get(index + window) {
                if *next_volt - *volt <= 3 {
                    let val = next_paths.get();
                    next_paths.set(val + paths.get());
                }
            }
        }
    }

    input.last().unwrap().1.get()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const LONG: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), (7 * 5));
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 8);
    }
    #[test]
    fn test_part2_long() {
        assert_eq!(solve_part2(generator(LONG).as_slice()), 19208);
    }
}
