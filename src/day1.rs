use std::num::ParseIntError;
use std::str::FromStr;
use std::vec;

#[derive(PartialEq, Debug)]
pub struct Elf {
    calories: Vec<usize>,
}

impl Elf {
    fn new<I>(calories: I) -> Elf
    where
        I: IntoIterator<Item = usize>,
    {
        Elf {
            calories: Vec::from_iter(calories.into_iter()),
        }
    }
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: Vec<_> = s
            .lines()
            .map(|x| x.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Elf::new(calories))
    }
}

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<Elf> {
    parse_err(input).unwrap()
}

pub fn parse_err(input: &str) -> Result<Vec<Elf>, ParseIntError> {
    input.split("\n\n").map(|x| Elf::from_str(x)).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Elf]) -> usize {
    input
        .iter()
        .map(|x| x.calories.iter().sum())
        .max()
        .expect("There are no elves")
}

#[aoc(day1, part2)]
pub fn part2(input: &[Elf]) -> usize {
    let mut elf_total_calories: Vec<_> = input.iter().map(|x| x.calories.iter().sum()).collect();
    elf_total_calories.sort();
    elf_total_calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Elf::new(vec![1000, 2000, 3000]),
                Elf::new(vec![4000]),
                Elf::new(vec![5000, 6000]),
                Elf::new(vec![7000, 8000, 9000]),
                Elf::new(vec![10000])
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2(&parse(TEST_INPUT)));
    }
}
