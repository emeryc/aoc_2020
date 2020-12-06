use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.bytes().fold(0u32, |acc, b| acc | 1 << (b - b'a')))
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(questionaires: &[Vec<u32>]) -> u32 {
    questionaires
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(0u32, |acc, person| acc | person)
                .count_ones()
        })
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(questionaires: &[Vec<u32>]) -> u32 {
    questionaires
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(u32::MAX, |acc, person| acc & person)
                .count_ones()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn sample_part1() {
        let questionaires = generate(INPUT);

        println!("{:?}", questionaires);

        assert_eq!(solve_part1(questionaires.as_slice()), 11)
    }

    #[test]
    fn sample_part2() {
        let questionaires = generate(INPUT);

        assert_eq!(solve_part2(questionaires.as_slice()), 6)
    }
}
