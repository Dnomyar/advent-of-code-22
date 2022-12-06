#![feature(control_flow_enum)]
use std::{collections::HashSet, fs, ops::ControlFlow};

fn read_file(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Unable to read the file");
}

fn hasDuplicateElements(window: &[(usize, char)]) -> bool {
    let uniqueSetOfElements: HashSet<char> = window.into_iter().map(|a| a.1).collect();
    return window.len() != uniqueSetOfElements.len();
}

fn logic(input: &str, numberOfChar: usize) -> Result<usize, String> {
    let charIndices: Vec<(usize, char)> = input.char_indices().collect();

    let maybeWindowIndex: ControlFlow<usize, usize> =
        charIndices
            .windows(numberOfChar)
            .try_fold(numberOfChar, |n, window| {
                if hasDuplicateElements(window) {
                    return ControlFlow::Continue(n + 1);
                } else {
                    return ControlFlow::Break(n);
                };
            });

    return match maybeWindowIndex {
        ControlFlow::Break(idx) => Ok(idx),
        _ => Err("could not find".to_string()),
    };
}

pub fn part1(input: &str) -> Result<usize, String> {
    return logic(input, 4);
}

pub fn part2(input: &str) -> Result<usize, String> {
    return logic(input, 14);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bvwbjplbgvbhsrlpgdmjqwftvncz_should_return_5() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(input), Ok(5));
    }

    #[test]
    fn nppdvjthqldpwncqszvftbrmjlhg_should_return_6() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(input), Ok(6));
    }

    #[test]
    fn nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg_should_return_10() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(input), Ok(10));
    }

    #[test]
    fn zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw_should_return_11() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(input), Ok(11));
    }

    #[test]
    fn part_1() {
        let input = read_file("resources/day6.txt");
        assert_eq!(part1(&input), Ok(1287));
    }

    #[test]
    fn part_2() {
        let input = read_file("resources/day6.txt");
        assert_eq!(part2(&input), Ok(3716));
    }
}
