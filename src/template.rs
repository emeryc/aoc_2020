type GeneratorType = String;

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<GeneratorType> {
    unimplemented!()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    0
}

#[aoc(day8, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 0);
    }
}
