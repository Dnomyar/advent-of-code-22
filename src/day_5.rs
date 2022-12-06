use std::{
    fs,
};

fn read_file(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Unable to read the file");
}

#[derive(PartialEq, Debug)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    number_of_elements_to_move: usize,
    from: usize,
    to: usize,
}
#[derive(PartialEq, Debug)]
struct Instructions {
    instructions: Vec<Instruction>,
}

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

fn parse_stack_line(line: &str) -> Vec<Option<char>> {
    let lineLen = line.len();
    let mut res = Vec::new();
    for idx in (1..(lineLen - 1)).step_by(4) {
        res.push(line.chars().nth(idx).filter(|e| e != &' '));
    }
    return res;
}

fn parseInstruction(line: &str) -> Instruction {
    let mut numbers = line
        .split_whitespace()
        .flat_map(|substr| substr.parse::<usize>());
    return Instruction {
        number_of_elements_to_move: numbers.next().unwrap(),
        from: numbers.next().unwrap(),
        to: numbers.next().unwrap(),
    };
}

fn parse(input: &str) -> (Stacks, Instructions) {
    let mut stackLines = Vec::new();
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.contains("[") {
            stackLines.push(parse_stack_line(line));
        } else if line.starts_with("move") {
            instructions.push(parseInstruction(line));
        }
    }
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut isInitiazed = false;
    // let mut map: HashMap<(usize, usize), &char> = HashMap::new();
    stackLines.reverse();
    stackLines.iter().enumerate().for_each(|(lineIndex, line)| {
        println!("line idx {}, line {:?}", lineIndex, line);

        if !isInitiazed {
            stacks = Vec::new();
            for _ in line {
                stacks.push(Vec::new());
            }
            println!("stack = {:?}", stacks);
            isInitiazed = true;
        }

        line.iter().enumerate().for_each(|(columIndex, element)| {
            element.into_iter().for_each(|e| {
                stacks[columIndex].push(*e);
            });
        });
    });
    return (
        Stacks { stacks: stacks },
        Instructions {
            instructions: instructions,
        },
    );
}

fn applyOneAtATime(instruction: Instruction, mut stacks: Stacks) -> Stacks {
    for _ in 0..(instruction.number_of_elements_to_move) {
        let element = stacks.stacks[instruction.from - 1].pop();
        element.map(|c| stacks.stacks[instruction.to - 1].push(c));
    }
    return stacks;
}

fn applySeveval(instruction: Instruction, mut stacks: Stacks) -> Stacks {
    let mut temp: Vec<char> = Vec::new();
    for _ in 0..(instruction.number_of_elements_to_move) {
        stacks.stacks[instruction.from - 1]
            .pop()
            .into_iter()
            .for_each(|c| temp.push(c))
    }
    temp.reverse();
    temp.into_iter()
        .for_each(|c| stacks.stacks[instruction.to - 1].push(c));
    return stacks;
}

pub fn partLogic(
    input: &str,
    apply: &dyn Fn(Instruction, Stacks) -> Stacks,
) -> Result<String, String> {
    let (stacks, instruction) = parse(input);
    let updatedStack = instruction
        .instructions
        .into_iter()
        .fold(stacks, |acc, i| apply(i, acc));
    let res: Vec<String> = updatedStack
        .stacks
        .into_iter()
        .flat_map(|mut stack| stack.pop())
        .map(|a| a.to_string())
        .collect();
    return Ok(res.join(""));
}

pub fn part1(input: &str) -> Result<String, String> {
    return partLogic(input, &applyOneAtATime);
}

pub fn part2(input: &str) -> Result<String, String> {
    return partLogic(input, &applySeveval);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_apply_an_instruction() {
        let stacks = Stacks {
            stacks: Vec::from([Vec::from(['N', 'Z']), Vec::from(['A'])]),
        };
        let instruction = Instruction {
            number_of_elements_to_move: 1,
            from: 1,
            to: 2,
        };
        assert_eq!(
            applyOneAtATime(instruction, stacks),
            Stacks {
                stacks: Vec::from([Vec::from(['N']), Vec::from(['A', 'Z'])])
            }
        );
    }

    #[test]
    fn should_apply_an_instruction_two_times() {
        let stacks = Stacks {
            stacks: Vec::from([Vec::from(['N', 'Z']), Vec::from(['A'])]),
        };
        let instruction = Instruction {
            number_of_elements_to_move: 2,
            from: 1,
            to: 2,
        };
        assert_eq!(
            applyOneAtATime(instruction, stacks),
            Stacks {
                stacks: Vec::from([Vec::new(), Vec::from(['A', 'Z', 'N'])])
            }
        );
    }

    #[test]
    fn should_parse() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(
            parse(input),
            (
                Stacks {
                    stacks: Vec::from([
                        Vec::from(['Z', 'N']),
                        Vec::from(['M', 'C', 'D']),
                        Vec::from(['P']),
                    ])
                },
                Instructions {
                    instructions: Vec::from([
                        Instruction {
                            number_of_elements_to_move: 1,
                            from: 2,
                            to: 1
                        },
                        Instruction {
                            number_of_elements_to_move: 3,
                            from: 1,
                            to: 3
                        },
                        Instruction {
                            number_of_elements_to_move: 2,
                            from: 2,
                            to: 1
                        },
                        Instruction {
                            number_of_elements_to_move: 1,
                            from: 1,
                            to: 2
                        },
                    ])
                }
            )
        )
    }

    #[test]
    fn part1_example() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part1(&input), Ok("CMZ".to_string()));
    }

    #[test]
    fn part2_example() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part2(&input), Ok("MCD".to_string()));
    }

    #[test]
    fn part1_resutl() {
        let input = read_file("resources/day5.txt");
        assert_eq!(part1(&input), Ok("QNNTGTPFN".to_string()));
    }

    #[test]
    fn part2_resutl() {
        let input = read_file("resources/day5.txt");
        assert_eq!(part2(&input), Ok("GGNPJBTTR".to_string()));
    }
}
