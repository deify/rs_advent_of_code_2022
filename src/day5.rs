#[derive(Debug, PartialEq, Eq)]
struct Stack(Vec<char>);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Move {
        stack: usize,
        from: usize,
        to: usize,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct CratePlan {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

fn parse_stack_drawing<I>(value: I) -> Vec<Stack> {}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> CratePlan {
    let mut lines = input.lines();
    let stack_drawing = lines.by_ref().take_while(|x| !x.trim().is_empty());

    println!("instructions");
    for b in lines {
        println!("{}", b);
    }

    CratePlan {
        stacks: vec![],
        instructions: vec![],
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &CratePlan) -> usize {
    0
}

#[aoc(day5, part2)]
pub fn part2(input: &CratePlan) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
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
                stacks: vec![
                    Stack(vec!['Z', 'N']),
                    Stack(vec!['M', 'C', 'D']),
                    Stack(vec!['P'])
                ],
                instructions: vec![
                    Instruction::Move {
                        stack: 3,
                        from: 1,
                        to: 3
                    },
                    Instruction::Move {
                        stack: 2,
                        from: 2,
                        to: 1
                    },
                    Instruction::Move {
                        stack: 1,
                        from: 1,
                        to: 2
                    },
                    Instruction::Move {
                        stack: 1,
                        from: 2,
                        to: 1
                    },
                ]
            },
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
