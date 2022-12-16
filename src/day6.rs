use itertools::Itertools;

fn find_unique_window(input: &str, window_size: usize) -> usize {
    let windows = input.as_bytes().windows(window_size);

    windows
        .enumerate()
        .find(|(_idx, x)| x.iter().unique().count() == window_size)
        .unwrap()
        .0
        + window_size
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_unique_window(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_unique_window(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_parse() {}

    #[test]
    fn test_part1() {
        assert_eq!(
            vec![7, 5, 6, 10, 11],
            TEST_INPUT.iter().map(|x| part1(x)).collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            vec![19, 23, 23, 29, 26],
            TEST_INPUT.iter().map(|x| part2(x)).collect::<Vec<_>>()
        );
    }
}
