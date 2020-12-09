use std::iter::Iterator;
type GeneratorType = i64;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

fn xmas_invalid(input: &[GeneratorType], consider: usize) -> i64 {
    *input
        .windows(consider + 1)
        .find(|window| {
            let (preamble, candidate) = window.split_at(consider);
            let candidate = candidate[0];
            for v in preamble {
                if preamble.contains(&(candidate - v)) && *v != (candidate - v) {
                    return false;
                }
            }
            true
        })
        .unwrap()
        .iter()
        .last()
        .unwrap()
}

fn weakness(input: &[GeneratorType], target: i64) -> i64 {
    // Solving this with brute force, will come up with a better answer later maybe.
    for window_size in 2..input.len() {
        if let Some(window) = input.windows(window_size).find(|window| {
            let check: i64 = window.iter().sum();
            check == target
        }) {
            return window.iter().max().unwrap() + window.iter().min().unwrap();
        }
    }
    0
}

#[aoc(day9, part1)]
fn solve_part1(input: &[GeneratorType]) -> i64 {
    xmas_invalid(input, 25)
}

#[aoc(day9, part2)]
fn solve_part2(input: &[GeneratorType]) -> i64 {
    weakness(input, xmas_invalid(input, 25))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part1() {
        assert_eq!(xmas_invalid(generator(SAMPLE).as_slice(), 5), 127);
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            weakness(
                generator(SAMPLE).as_slice(),
                xmas_invalid(generator(SAMPLE).as_slice(), 5)
            ),
            62
        );
    }
}
