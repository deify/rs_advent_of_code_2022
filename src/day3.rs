use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    InvalidItemChar,
    UnevenNumberOfItems,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct ValidItem {
    item: char,
}

impl ValidItem {
    fn score(&self) -> u8 {
        if self.item.is_ascii_lowercase() {
            (self.item as u8) - b'a' + 1
        } else {
            (self.item as u8) - b'A' + 27
        }
    }
}

impl TryFrom<char> for ValidItem {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_lowercase() || value.is_ascii_uppercase() {
            Ok(ValidItem { item: value })
        } else {
            Err(ParseError::InvalidItemChar)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Compartment(Vec<ValidItem>);

impl FromStr for Compartment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .chars()
            .map(ValidItem::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Compartment(items))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rucksack {
    comp1: Compartment,
    comp2: Compartment,
}

impl FromStr for Rucksack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(ParseError::UnevenNumberOfItems);
        }

        let (comp1, comp2) = s.split_at(s.len() / 2);
        Ok(Rucksack {
            comp1: Compartment::from_str(comp1).expect("invalid compartment"),
            comp2: Compartment::from_str(comp2).expect("invalid compartment"),
        })
    }
}

impl Rucksack {
    fn common_item(&self) -> Option<&ValidItem> {
        let set: HashSet<_> = self.comp1.0.iter().collect();
        self.comp2.0.iter().find(|c| set.contains(c))
    }

    fn iter_all(&self) -> impl Iterator<Item = &ValidItem> {
        self.comp1.0.iter().chain(self.comp2.0.iter())
    }
}

#[derive(Debug)]
struct ElfGroup<'a>(Vec<&'a Rucksack>);

impl<'a> ElfGroup<'a> {
    fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a Rucksack>,
    {
        ElfGroup(iter.into_iter().collect())
    }

    fn find_badge(&self) -> Option<ValidItem> {
        let sets: Vec<HashSet<_>> = self.0.iter().map(|x| x.iter_all().collect()).collect();
        let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

        assert_eq!(intersection.len(), 1);
        let item = intersection.iter().next()?;

        Some(**item)
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|x| Rucksack::from_str(x.trim()).expect("failed to parse Rucksack from String"))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .map(|x| x.common_item().unwrap().score() as u32)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Rucksack]) -> u32 {
    let groups: Vec<_> = input.chunks_exact(3).map(ElfGroup::new).collect();
    groups
        .iter()
        .map(|x| x.find_badge().unwrap().score() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Rucksack {
                    comp1: Compartment::from_str("vJrwpWtwJgWr").unwrap(),
                    comp2: Compartment::from_str("hcsFMMfFFhFp").unwrap(),
                },
                Rucksack {
                    comp1: Compartment::from_str("jqHRNqRjqzjGDLGL").unwrap(),
                    comp2: Compartment::from_str("rsFMfFZSrLrFZsSL").unwrap(),
                },
                Rucksack {
                    comp1: Compartment::from_str("PmmdzqPrV").unwrap(),
                    comp2: Compartment::from_str("vPwwTWBwg").unwrap(),
                },
                Rucksack {
                    comp1: Compartment::from_str("wMqvLMZHhHMvwLH").unwrap(),
                    comp2: Compartment::from_str("jbvcjnnSBnvTQFn").unwrap(),
                },
                Rucksack {
                    comp1: Compartment::from_str("ttgJtRGJ").unwrap(),
                    comp2: Compartment::from_str("QctTZtZT").unwrap(),
                },
                Rucksack {
                    comp1: Compartment::from_str("CrZsJsPPZsGz").unwrap(),
                    comp2: Compartment::from_str("wwsLwLmpwMDw").unwrap(),
                }
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(157, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(70, part2(&parse(TEST_INPUT)));
    }
}
