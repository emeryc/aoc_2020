use eyre::{eyre, Error};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type GeneratorType = Label;

#[derive(Debug)]
struct Label {
    ingridients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Label {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" (");
        let ingridients = s
            .next()
            .ok_or_else(|| eyre!("No Ingridients?"))?
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        let allergens = s
            .next()
            .ok_or_else(|| eyre!("No Allergens"))?
            .strip_suffix(')')
            .ok_or_else(|| eyre!("No closing )"))?
            .strip_prefix("contains ")
            .ok_or_else(|| eyre!("No Contains"))?
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Ok(Label {
            ingridients,
            allergens,
        })
    }
}

#[aoc_generator(day21)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|s| s.parse::<Label>().unwrap())
        .collect()
}

#[aoc(day21, part1)]
fn solve_part1(input: &[GeneratorType]) -> usize {
    let mut map: HashMap<&str, Vec<&HashSet<String>>> = HashMap::new();
    let mut ingridents = HashSet::new();
    for label in input {
        for allergen in label.allergens.iter() {
            map.entry(allergen)
                .or_insert(Vec::new())
                .push(&label.ingridients);
            for ing in label.ingridients.iter() {
                ingridents.insert(ing.as_str());
            }
        }
    }

    let mut possible: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (allergen, ingredients) in map.iter() {
        possible.insert(
            allergen,
            ingredients
                .iter()
                .cloned()
                .flat_map(|a| a.iter().map(|s| s.as_str()))
                .collect(),
        );
    }

    let mut impossible: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (allergen, ingredients) in map.iter() {
        impossible.insert(
            allergen,
            ingredients
                .iter()
                .cartesian_product(ingredients.iter())
                .flat_map(|(a, b)| HashSet::symmetric_difference(a, b).map(|s| s.as_str()))
                .collect(),
        );
    }

    let mut not_safe = HashSet::new();
    for allergen in map.keys() {
        let maybe = possible
            .get(allergen)
            .unwrap()
            .difference(impossible.get(allergen).unwrap());
        for m in maybe {
            not_safe.insert(*m);
        }
    }

    let safe = ingridents.difference(&not_safe).collect::<HashSet<_>>();

    input
        .iter()
        .map(|l| {
            l.ingridients
                .iter()
                .filter(|i| safe.contains(&i.as_str()))
                .count()
        })
        .sum()
}

#[aoc(day21, part2)]
fn solve_part2(input: &[GeneratorType]) -> String {
    let mut map: HashMap<&str, Vec<&HashSet<String>>> = HashMap::new();
    let mut ingridents = HashSet::new();
    for label in input {
        for allergen in label.allergens.iter() {
            map.entry(allergen)
                .or_insert(Vec::new())
                .push(&label.ingridients);
            for ing in label.ingridients.iter() {
                ingridents.insert(ing.as_str());
            }
        }
    }

    let mut possible: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (allergen, ingredients) in map.iter() {
        possible.insert(
            allergen,
            ingredients
                .iter()
                .cloned()
                .flat_map(|a| a.iter().map(|s| s.as_str()))
                .collect(),
        );
    }

    let mut impossible: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (allergen, ingredients) in map.iter() {
        impossible.insert(
            allergen,
            ingredients
                .iter()
                .cartesian_product(ingredients.iter())
                .flat_map(|(a, b)| HashSet::symmetric_difference(a, b).map(|s| s.as_str()))
                .collect(),
        );
    }

    let mut not_safe = HashSet::new();
    for allergen in map.keys() {
        let maybe = possible
            .get(allergen)
            .unwrap()
            .difference(impossible.get(allergen).unwrap());
        for m in maybe {
            not_safe.insert(*m);
        }
    }

    let mut a = map
        .iter()
        .map(|(allergen, words_sets)| {
            (
                *allergen,
                not_safe
                    .iter()
                    .filter(|w| words_sets.iter().all(|s| s.contains(**w)))
                    .copied()
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<&str, HashSet<&str>>>();
    let mut result: HashSet<(&str, &str)> = HashSet::new();
    loop {
        for x in result.iter() {
            a.remove(x.0);
            for (_, p) in a.iter_mut() {
                p.remove(x.1);
            }
        }
        if a.is_empty() {
            break;
        }
        for (allergen, possible) in a.iter() {
            if possible.len() == 1 {
                result.insert((allergen, possible.iter().last().unwrap()));
            }
        }
    }

    let mut result = result.into_iter().collect::<Vec<_>>();
    result.sort_by_key(|(k, _)| *k);

    result.iter().map(|(_, v)| v).join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 5);
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(generator(SAMPLE).as_slice()),
            "mxmxvkd,sqjhc,fvjkl"
        );
    }
}
