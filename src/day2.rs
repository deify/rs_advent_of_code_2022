use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum RockPaperScissorsResult {
    Lost,
    Draw,
    Win,
}

impl RockPaperScissorsResult {
    fn score(&self) -> usize {
        match self {
            RockPaperScissorsResult::Lost => 0,
            RockPaperScissorsResult::Draw => 3,
            RockPaperScissorsResult::Win => 6,
        }
    }
}

impl FromStr for RockPaperScissorsResult {
    type Err = RockPaperScissorsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RockPaperScissorsResult::Lost),
            "Y" => Ok(RockPaperScissorsResult::Draw),
            "Z" => Ok(RockPaperScissorsResult::Win),
            _ => Err(RockPaperScissorsParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
pub struct RockPaperScissorsParseError;

impl FromStr for RockPaperScissors {
    type Err = RockPaperScissorsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RockPaperScissors::Rock),
            "B" | "Y" => Ok(RockPaperScissors::Paper),
            "C" | "Z" => Ok(RockPaperScissors::Scissor),
            _ => Err(RockPaperScissorsParseError),
        }
    }
}

impl RockPaperScissors {
    fn strategy_p1(&self, other: &Self) -> RockPaperScissorsResult {
        match self {
            RockPaperScissors::Rock => match other {
                &RockPaperScissors::Rock => RockPaperScissorsResult::Draw,
                RockPaperScissors::Paper => RockPaperScissorsResult::Lost,
                RockPaperScissors::Scissor => RockPaperScissorsResult::Win,
            },
            RockPaperScissors::Paper => match other {
                &RockPaperScissors::Rock => RockPaperScissorsResult::Win,
                RockPaperScissors::Paper => RockPaperScissorsResult::Draw,
                RockPaperScissors::Scissor => RockPaperScissorsResult::Lost,
            },
            RockPaperScissors::Scissor => match other {
                &RockPaperScissors::Rock => RockPaperScissorsResult::Lost,
                RockPaperScissors::Paper => RockPaperScissorsResult::Win,
                RockPaperScissors::Scissor => RockPaperScissorsResult::Draw,
            },
        }
    }

    fn score(&self) -> usize {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissor => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StrategyGuide {
    opponent: RockPaperScissors,
    player_p1: RockPaperScissors,
    result_p2: RockPaperScissorsResult,
}

impl StrategyGuide {
    fn score_p1(&self) -> usize {
        let result = self.player_p1.strategy_p1(&self.opponent);
        result.score() + self.player_p1.score()
    }

    fn strategy_p2(&self) -> RockPaperScissors {
        match self.opponent {
            RockPaperScissors::Rock => match self.result_p2 {
                RockPaperScissorsResult::Win => RockPaperScissors::Paper,
                RockPaperScissorsResult::Draw => RockPaperScissors::Rock,
                RockPaperScissorsResult::Lost => RockPaperScissors::Scissor,
            },
            RockPaperScissors::Paper => match self.result_p2 {
                RockPaperScissorsResult::Win => RockPaperScissors::Scissor,
                RockPaperScissorsResult::Draw => RockPaperScissors::Paper,
                RockPaperScissorsResult::Lost => RockPaperScissors::Rock,
            },
            RockPaperScissors::Scissor => match self.result_p2 {
                RockPaperScissorsResult::Win => RockPaperScissors::Rock,
                RockPaperScissorsResult::Draw => RockPaperScissors::Scissor,
                RockPaperScissorsResult::Lost => RockPaperScissors::Paper,
            },
        }
    }

    fn score_p2(&self) -> usize {
        self.strategy_p2().score() + self.result_p2.score()
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<StrategyGuide> {
    input
        .lines()
        .map(|x| x.trim().split_once(' ').expect("Invalid format"))
        .map(|(a, b)| StrategyGuide {
            opponent: RockPaperScissors::from_str(a.trim()).expect("bad input"),
            player_p1: RockPaperScissors::from_str(b.trim()).expect("bad input"),
            result_p2: RockPaperScissorsResult::from_str(b.trim()).expect("bad input"),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[StrategyGuide]) -> usize {
    input.iter().map(|x| x.score_p1()).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[StrategyGuide]) -> usize {
    input.iter().map(|x| x.score_p2()).sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                StrategyGuide {
                    opponent: RockPaperScissors::Rock,
                    player_p1: RockPaperScissors::Paper,
                    result_p2: RockPaperScissorsResult::Draw
                },
                StrategyGuide {
                    opponent: RockPaperScissors::Paper,
                    player_p1: RockPaperScissors::Rock,
                    result_p2: RockPaperScissorsResult::Lost
                },
                StrategyGuide {
                    opponent: RockPaperScissors::Scissor,
                    player_p1: RockPaperScissors::Scissor,
                    result_p2: RockPaperScissorsResult::Win
                }
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(&parse(TEST_INPUT)));
    }
}
