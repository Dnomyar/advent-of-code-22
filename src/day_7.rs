use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1, not_line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

#[derive(Debug)]
enum Command {
    Cd { to: String },
    Dir { name: String },
    File { name: String, size: u64 },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ParentDir {
    name: String,
}

impl TryFrom<FileType> for ParentDir {
    type Error = String;

    fn try_from(value: FileType) -> Result<Self, Self::Error> {
        match value {
            FileType::Dir { name } => Ok(ParentDir { name: name }),
            _ => Err("is not a dir".to_string()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum FileType {
    Dir { name: String },
    File { name: String, size: u64 },
}
trait FileTypeOps {
    fn is_dir(&self) -> bool;
    fn get_file_size(&self) -> Option<&u64>;
}

impl FileTypeOps for FileType {
    fn is_dir(&self) -> bool {
        match self {
            FileType::Dir { name: _ } => true,
            _ => false,
        }
    }

    fn get_file_size(&self) -> Option<&u64> {
        match self {
            FileType::File { name: _, size } => Some(size),
            _ => None,
        }
    }
}

fn not_line_ending2(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    let cd = preceded(
        tag("$ cd "),
        map_res(not_line_ending2, |e: &str| -> Result<Vec<Command>, &str> {
            Ok(Vec::from([Command::Cd { to: e.to_string() }]))
        }),
    );

    let ls_options_dir = preceded(
        tag("dir "),
        map_res(not_line_ending2, |e: &str| -> Result<Vec<Command>, &str> {
            Ok(Vec::from([Command::Dir {
                name: e.to_string(),
            }]))
        }),
    );
    let ls_option_file = map_res(
        tuple((
            map_res(digit1, |s: &str| s.parse::<u64>()),
            char(' '),
            not_line_ending2,
        )),
        |(size, _, name)| -> Result<Vec<Command>, &str> {
            Ok(Vec::from([Command::File {
                name: name.to_string(),
                size: size,
            }]))
        },
    );

    let ls_options = alt((ls_options_dir, ls_option_file));

    let ls_options1 = map_res(separated_list1(char('\n'), ls_options), |commands| {
        Ok::<Vec<Command>, &str>(commands.into_iter().flatten().collect())
    });

    let ls = preceded(tag("$ ls\n"), ls_options1);

    let options = alt((cd, ls));

    return map_res(separated_list1(char('\n'), options), |commands| {
        Ok::<Vec<Command>, &str>(commands.into_iter().flatten().collect())
    })(input);
}

fn create_relataionship_from_commands(commands: Vec<Command>) -> HashMap<FileType, ParentDir> {
    let mut current_directory_deque: VecDeque<String> = VecDeque::new();
    let mut child_parent_relationship: HashMap<FileType, ParentDir> = HashMap::new();
    for command in commands {
        let parent_dir = ParentDir {
            name: (&current_directory_deque).into_iter().rev().join("/"),
        };
        match command {
            Command::Cd { to } => {
                if to == ".." {
                    current_directory_deque.pop_front();
                } else {
                    current_directory_deque.push_front(to);
                }
            }
            Command::Dir { name } => {
                child_parent_relationship.insert(
                    FileType::Dir {
                        name: parent_dir.name.to_string() + "/" + &name,
                    },
                    parent_dir,
                );
            }
            Command::File { name, size } => {
                child_parent_relationship.insert(
                    FileType::File {
                        name: parent_dir.name.to_string() + "/" + &name,
                        size: size,
                    },
                    parent_dir,
                );
            }
        }
    }
    return child_parent_relationship;
}

fn create_graph_from_relationships(
    file_parent_map: HashMap<FileType, ParentDir>,
) -> HashMap<ParentDir, HashSet<FileType>> {
    let map_of_vec: HashMap<ParentDir, Vec<FileType>> = file_parent_map
        .clone()
        .into_iter()
        .map(|(k, v)| (v, k))
        .into_group_map();

    return map_of_vec
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect::<HashSet<FileType>>()))
        .collect();
}

fn compute_directory_sizes(
    graph: HashMap<ParentDir, HashSet<FileType>>,
) -> HashMap<ParentDir, u64> {
    let mut edges_to_visit = VecDeque::from([ParentDir {
        name: "/".to_string(),
    }]);

    let mut directory_sizes: HashMap<ParentDir, u64> = HashMap::new();

    println!("graph keys {:?}", graph.keys());

    while !edges_to_visit.is_empty() {
        let front = edges_to_visit.front().unwrap();
        println!("front {:?}", front);
        let children = graph.get(front).unwrap(); //.unwrap_or(&empty);
        let children_directories: HashSet<_> = children
            .into_iter()
            .flat_map(|d| match d {
                FileType::Dir { name } => Some(ParentDir {
                    name: name.to_string(),
                }),
                _ => None,
            })
            .filter(|child| !directory_sizes.contains_key(child))
            .collect();

        if children_directories.is_empty() {
            let total_size: u64 = children
                .into_iter()
                .flat_map(|child| match child {
                    FileType::Dir { name } => directory_sizes.get(&ParentDir {
                        name: name.to_string(),
                    }),
                    _file @ FileType::File { name: _, size: _ } => child.get_file_size(),
                })
                .sum();
            directory_sizes.insert(front.clone(), total_size);
            // res = Some(total_size);
            edges_to_visit.pop_front();
        } else {
            children_directories
                .into_iter()
                .for_each(|directory| edges_to_visit.push_front(directory));
        }
    }
    return directory_sizes;
}

pub fn part1(input: &str) -> Result<u64, String> {
    let commands = parse_input(input).unwrap().1;
    let child_parent_relationship = create_relataionship_from_commands(commands);

    println!();
    child_parent_relationship
        .clone()
        .into_iter()
        .for_each(|A| println!("{:?}", A));
    println!();

    let graph = create_graph_from_relationships(child_parent_relationship);

    println!();
    graph.clone().into_iter().for_each(|A| println!("{:?}", A));
    println!();

    let directory_sizes = compute_directory_sizes(graph);

    let sum_small_directories: u64 = directory_sizes
        .into_iter()
        .filter(|(_k, v)| v < &100000)
        .map(|(_k, v)| v)
        .sum();

    return Ok(sum_small_directories);
}

pub fn part2(_input: &str) -> Result<usize, String> {
    return todo!();
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(part1(input), Ok(95437));
    }

    fn read_file(file_name: &str) -> String {
        return fs::read_to_string(file_name).expect("Unable to read the file");
    }

    // #[test]
    fn should_work_with_nested_directories() {
        let input = "$ cd /
$ ls
dir a
$ cd a
$ ls
dir b
$ cd b
$ ls
29116 f";
        assert_eq!(part1(input), Ok(95437));
    }

    #[test]
    fn part1_result() {
        let input = read_file("resources/day7.txt");
        assert_eq!(part1(&input), Ok(0));
    }
}
