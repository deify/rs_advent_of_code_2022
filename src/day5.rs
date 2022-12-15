use core::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Self {
        Stack(vec![])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(mv) => {
                let from = &mut self.0[mv.from];
                let drained : Vec<_> = from.0.drain(from.0.len() - mv.count..).collect();

                let to = &mut self.0[mv.to];
                drained.iter().for_each(|x| to.0.push(*x));
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MoveArguments {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Move(MoveArguments),
}

#[derive(Debug, PartialEq, Eq)]
enum InstructionParseError {
    InvalidInstruction,
}

impl From<ParseIntError> for InstructionParseError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidInstruction
    }
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_args: Vec<_> = s.split(' ').collect();
        match instruction_args[..] {
            ["move", count, "from", from, "to", to] => {
                let count = count.parse()?;
                let from = from.parse()?;
                let to = to.parse()?;

                Ok(Instruction::Move(MoveArguments { count, from, to }))
            }
            _ => Err(InstructionParseError::InvalidInstruction),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CratePlan {
    stacks: Stacks,
    instructions: Vec<Instruction>,
}

fn parse_stack_drawing<'a, I>(mut stack_drawing: I) -> Option<Stacks>
where
    I: Iterator<Item = &'a str>,
{
    let stack_num = stack_drawing
        .next()?
        .split_ascii_whitespace()
        .skip_while(|x| x.is_empty())
        .count();

    let mut stacks: Stacks = Stacks(Vec::with_capacity(stack_num));
    stacks.0.resize_with(stack_num, Stack::new);

    for line in stack_drawing {
        println!("{}", line);
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, x)| !x.is_ascii_whitespace())
            .for_each(|(idx, value)| {
                println!("pushing {} to stack {}", value, idx);
                stacks.0[idx].0.push(value);
            });
    }

    Some(stacks)
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> CratePlan {
    let mut lines = input.lines();
    let stack_drawing = lines.by_ref().take_while(|x| !x.trim().is_empty());

    let stacks = parse_stack_drawing(stack_drawing.collect::<Vec<_>>().into_iter().rev())
        .expect("failed to parse stack drawing");

    let instructions = lines
        .map(|x| Instruction::from_str(x).expect("failed to parse exception"))
        .collect();

    CratePlan {
        stacks,
        instructions,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &CratePlan) -> String {
    let mut stacks = input.stacks.clone();
    input
        .instructions
        .iter()
        .for_each(|i| stacks.run_instruction(i));

    "".to_string()
}

#[aoc(day5, part2)]
pub fn part2(input: &CratePlan) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
\x20\x20\x20\x20[D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse() {
        assert_eq!(
            CratePlan {
                stacks: Stacks(vec![
                    Stack(vec!['Z', 'N']),
                    Stack(vec!['M', 'C', 'D']),
                    Stack(vec!['P'])
                ]),
                instructions: vec![
                    Instruction::Move(MoveArguments {
                        count: 1,
                        from: 2,
                        to: 1
                    }),
                    Instruction::Move(MoveArguments {
                        count: 3,
                        from: 1,
                        to: 3
                    }),
                    Instruction::Move(MoveArguments {
                        count: 2,
                        from: 2,
                        to: 1
                    }),
                    Instruction::Move(MoveArguments {
                        count: 1,
                        from: 1,
                        to: 2
                    }),
                ]
            },
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(
            Instruction::from_str("move 3 from 1 to 3"),
            Ok(Instruction::Move(MoveArguments {
                count: 3,
                from: 1,
                to: 3
            }))
        );

        assert_eq!(
            Instruction::from_str(""),
            Err(InstructionParseError::InvalidInstruction)
        );

        assert_eq!(
            Instruction::from_str("move a from 1 to 2"),
            Err(InstructionParseError::InvalidInstruction)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!("CMZ", part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
