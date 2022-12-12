use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Range(usize, usize);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRangeFormat,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(ParseError::InvalidRangeFormat)?;
        let a = a.parse().map_err(|_| ParseError::InvalidRangeFormat)?;
        let b = b.parse().map_err(|_| ParseError::InvalidRangeFormat)?;

        assert!(a <= b);
        Ok(Range(a, b))
    }
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| {
            (
                Range::from_str(a.trim()).unwrap(),
                Range::from_str(b.trim()).unwrap(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range, Range)]) -> usize {
    input.iter().filter(|(a, b)| a.overlaps(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                (Range(2, 4), Range(6, 8)),
                (Range(2, 3), Range(4, 5)),
                (Range(5, 7), Range(7, 9)),
                (Range(2, 8), Range(3, 7)),
                (Range(6, 6), Range(4, 6)),
                (Range(2, 6), Range(4, 8))
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(&parse(TEST_INPUT)));
    }
}
