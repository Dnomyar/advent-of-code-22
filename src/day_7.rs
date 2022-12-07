use std::{
    collections::{HashMap, VecDeque},
    io::ErrorKind,
};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1, not_line_ending},
    combinator::map_res,
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

#[derive(Debug)]
enum Command {
    Cd { to: String },
    Dir { name: String },
    File { name: String, size: u32 },
}

fn not_line_ending2(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

pub fn part1(input: &str) -> Result<usize, String> {
    let current_directory_deque: VecDeque<&str> = VecDeque::new();
    let child_parent_relationship: HashMap<&str, &str> = HashMap::new();

    /*
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    */
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ ls
584 i
$ cd ..
$ cd ..";

    // let a = |e: &str| not_line_ending(e);
    // let cd_to = map_res(not_line_ending2, |e: &str| -> Result<Command, &str> {
    //     Ok(Command::Cd { to: e.to_string() })
    // })(input);

    let cd = preceded(
        tag("$ cd "),
        map_res(not_line_ending2, |e: &str| -> Result<Vec<Command>, &str> {
            Ok(Vec::from([Command::Cd { to: e.to_string() }]))
        }),
    );

    let ls_options_dir = preceded(
        tag("dir"),
        map_res(not_line_ending2, |e: &str| -> Result<Vec<Command>, &str> {
            Ok(Vec::from([Command::Dir {
                name: e.to_string(),
            }]))
        }),
    );
    let ls_option_file = map_res(
        tuple((
            map_res(digit1, |s: &str| s.parse::<u32>()),
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

    let res = separated_list1(char('\n'), options)(input);
    // // let res = cd.map(|(to, _)| Command::Cd { to: to.to_string() });
    println!("RES = {:?}", res);

    // let cd = tag("$ cd")(input);

    // separated_list1(char("", f)

    // tag("$ cd ").map(|a| a);
    // alt(
    //     tag("$ cd ").map(|a|a),
    //     // map_res(s)
    // )
    return todo!();
}

pub fn part2(input: &str) -> Result<usize, String> {
    return todo!();
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::*;

    // #[test]
    // fn nom_test() {
    //     assert_eq!(true, true);
    // }

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
}
