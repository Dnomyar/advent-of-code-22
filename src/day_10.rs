use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

type DelayBeforeInterpretation = i32;

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.contains("noop") {
                Instruction::Noop
            } else {
                let (_, number_string) = line.split_once(" ").unwrap();
                let number = number_string.parse::<i32>().unwrap();
                Instruction::Addx(number)
            }
        })
        .collect()
}

fn prefix_with_delay_before_interpretation(
    instructions: Vec<Instruction>,
) -> VecDeque<(DelayBeforeInterpretation, Instruction)> {
    VecDeque::from(
        instructions
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::Noop => (0, instruction),
                Instruction::Addx(_) => (1, instruction),
            })
            .collect::<Vec<(DelayBeforeInterpretation, Instruction)>>(),
    )
}

fn interpret_instructions(instructions: Vec<Instruction>) -> HashMap<i32, i32> {
    let mut instuction_deque: VecDeque<(DelayBeforeInterpretation, Instruction)> =
        prefix_with_delay_before_interpretation(instructions);

    let mut register_history = HashMap::new();
    let mut register = 1;
    for cycle in (1..) {
        register_history.insert(cycle, register);
        match instuction_deque.pop_front() {
            Some((count, instruction)) if count == 0 => match instruction {
                Instruction::Noop => (),
                Instruction::Addx(value) => {
                    register += value;
                }
            },
            Some((count, instruction)) => {
                instuction_deque.push_front((count - 1, instruction));
            }
            None => break,
        };
    }
    register_history
}

fn part1(input: &str) -> i32 {
    let instructions: Vec<Instruction> = parse(input);

    let registry_history = interpret_instructions(instructions);

    let intersting_cycles = vec![20, 60, 100, 140, 180, 220];
    intersting_cycles
        .into_iter()
        .map(|cycle| cycle * registry_history[&cycle])
        .sum()
}

mod tests {

    use std::{
        collections::{HashMap, VecDeque},
        fs,
    };

    use super::*;

    #[test]
    fn should_intepret_instructions() {
        let instructions = Vec::from([
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ]);
        let res = interpret_instructions(instructions);
        assert_eq!(res.get(&1), Some(&1));
        assert_eq!(res.get(&2), Some(&1));
        assert_eq!(res.get(&3), Some(&1));
        assert_eq!(res.get(&4), Some(&4));
        assert_eq!(res.get(&5), Some(&4));
        assert_eq!(res.get(&6), Some(&-1));
    }

    #[test]
    fn part1_example() {
        let input = &read_file("resources/day10-example.txt");
        assert_eq!(part1(input), 13140);
    }

    #[test]
    fn part1_res() {
        let input = &read_file("resources/day10.txt");
        assert_eq!(part1(input), 11220);
    }

    fn read_file(file_name: &str) -> String {
        return fs::read_to_string(file_name).expect("Unable to read the file");
    }
}
