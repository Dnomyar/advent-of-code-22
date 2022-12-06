use std::fs;

fn read_file(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Unable to read the file");
}
#[derive(PartialEq)]
pub enum Options {
    Rock,
    Paper,
    Scissors,
}

pub fn part1(input: &str) -> Option<u32> {
    // TODO: assumption, the input is correct
    let games = input.split('\n').map(|line| {
        line.split(' ').flat_map(|letter| match letter {
            "A" | "X" => Some(Options::Rock),
            "B" | "Y" => Some(Options::Paper),
            "C" | "Z" => Some(Options::Scissors),
            letter => None,
        })
    });

    return Some(
        games
            .map(|round| {
                let mut enumerated = round;
                let opponentPlayed = enumerated.next();
                let iPlayed = enumerated.next();

                let looseDrawWinScore = if opponentPlayed == iPlayed {
                    3
                } else if (opponentPlayed == Some(Options::Scissors)
                    && iPlayed == Some(Options::Paper))
                    || (opponentPlayed == Some(Options::Paper) && iPlayed == Some(Options::Rock))
                    || (opponentPlayed == Some(Options::Rock) && iPlayed == Some(Options::Scissors))
                {
                    0
                } else {
                    6
                };

                let whatIPlayedScore = match iPlayed {
                    Some(Options::Rock) => 1,
                    Some(Options::Paper) => 2,
                    Some(Options::Scissors) => 3,
                    _ => 0,
                };

                return looseDrawWinScore + whatIPlayedScore;
            })
            .sum(),
    );
}

pub fn part2() -> Option<u32> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "A Y
        B X
        C Z";
        assert_eq!(part1(input), Some(15));
    }

    #[test]
    fn part1_result() {
        let input = read_file("resources/day2.txt");
        assert_eq!(part1(&input), Some(13924));
    }

    // #[test]
    fn part2_test() {
        assert_eq!(part2(), Some(199628));
    }
}
