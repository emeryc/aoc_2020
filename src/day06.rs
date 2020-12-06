use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.bytes().collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(questionaires: &[Vec<Vec<u8>>]) -> u64 {
    questionaires
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.iter())
                .collect::<HashSet<_>>()
                .len() as u64
        })
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(questionaires: &[Vec<Vec<u8>>]) -> u64 {
    questionaires
        .iter()
        .map(|group| {
            group
                .iter()
                .skip(1)
                .fold(
                    group.first().unwrap().iter().collect::<HashSet<_>>(),
                    |acc, person| {
                        acc.intersection(&person.iter().collect::<HashSet<_>>())
                            .cloned()
                            .collect()
                    },
                )
                .len() as u64
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
