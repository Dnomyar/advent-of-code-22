use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Either;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum VisibilityDirections {
    Top,
    Bottom,
    Left,
    Right,
}
#[derive(PartialEq, Debug, Clone)]
struct VisibleFrom {
    directions: HashSet<VisibilityDirections>,
}

struct MissingVisibilityInformation {
    directions: HashSet<VisibilityDirections>,
}

#[derive(Debug)]
struct Forest {
    trees: HashMap<Coords, u32>,
    number_of_lines: usize,
    number_of_colunms: usize,
}

type Coords = (usize, usize);
type TreesVisibility = HashMap<Coords, VisibleFrom>;

fn parse_into_forest(input: &str) -> Forest {
    let mut forest: HashMap<(usize, usize), u32> = HashMap::new();
    let mut number_of_lines = 0;
    let mut number_of_columns = 0;
    input
        .split('\n')
        .into_iter()
        .enumerate()
        .for_each(|(lineIdx, line)| {
            number_of_lines += 1;
            number_of_columns = 0;
            line.char_indices().for_each(|(colIdx, col)| {
                number_of_columns += 1;
                forest.insert((lineIdx, colIdx), col.to_digit(10).unwrap());
            })
        });
    return Forest {
        trees: forest,
        number_of_colunms: number_of_columns,
        number_of_lines: number_of_lines,
    };
}

fn default_visibility(forest: &Forest, tree: Coords) -> VisibleFrom {
    let (lineIdx, colIdx) = tree;
    let mut visibility = HashSet::new();
    if (lineIdx <= 0) {
        visibility.insert(VisibilityDirections::Top);
    } else if lineIdx >= forest.number_of_lines - 1 {
        visibility.insert(VisibilityDirections::Bottom);
    }
    if (colIdx <= 0) {
        visibility.insert(VisibilityDirections::Left);
    } else if colIdx >= forest.number_of_colunms - 1 {
        visibility.insert(VisibilityDirections::Right);
    }
    return VisibleFrom {
        directions: visibility,
    };
}

fn find_tree_visibiilty(
    forest: Forest,
    trees_visibility: TreesVisibility,
    tree: Coords,
) -> Either<MissingVisibilityInformation, VisibleFrom> {
    return todo!();
}

fn get_unexplored_directions(visible_from: &VisibleFrom) -> HashSet<VisibilityDirections> {
    let all_directions: HashSet<VisibilityDirections> = HashSet::from([
        VisibilityDirections::Bottom,
        VisibilityDirections::Top,
        VisibilityDirections::Left,
        VisibilityDirections::Right,
    ]);

    let res: HashSet<_> = all_directions
        .difference(&visible_from.directions)
        .collect();

    return res.into_iter().map(|v| v.clone()).collect();
}

fn visit_forest(forest: Forest) -> usize {
    let mut to_visit: VecDeque<(Coords, VisibilityDirections)> = VecDeque::new();

    for line_idx in 0..forest.number_of_lines {
        to_visit.push_back(((line_idx, 0), VisibilityDirections::Right));
        to_visit.push_back((
            (line_idx, forest.number_of_colunms - 1),
            VisibilityDirections::Left,
        ));
    }
    for colunm_idx in 0..forest.number_of_colunms {
        to_visit.push_back(((0, colunm_idx), VisibilityDirections::Bottom));
        to_visit.push_back((
            (forest.number_of_colunms - 1, colunm_idx),
            VisibilityDirections::Top,
        ));
    }

    println!("to_visit {:?}", to_visit);

    let mut visible = HashSet::new();

    to_visit.clone().into_iter().for_each(|(coord, _)| {
        visible.insert(coord);
        return ();
    });
    println!("visible {:?}", visible.len());

    let mut count = 0;
    while !to_visit.is_empty() && count < 100 {
        let (tree_to_visit @ (lineIdx, colIdx), direction) = to_visit.pop_front().unwrap().clone();

        match forest.trees.get(&tree_to_visit) {
            Some(tree_to_visit_hight) => {
                println!("");
                println!(
                    "tree_to_visit {:?} {:?} {:?}",
                    tree_to_visit, direction, tree_to_visit_hight,
                );

                // println!("visible {:?}", visible);

                let next_coord_to_check = match direction {
                    VisibilityDirections::Bottom => Some((lineIdx + 1, colIdx)),
                    VisibilityDirections::Top => {
                        lineIdx.checked_sub(1).map(|lineIdx| (lineIdx, colIdx))
                    }
                    VisibilityDirections::Left => {
                        colIdx.checked_sub(1).map(|colIdx| (lineIdx, colIdx))
                    }
                    VisibilityDirections::Right => Some((lineIdx, colIdx + 1)),
                };
                // println!("next_coord_to_check {:?}", next_coord_to_check);

                match next_coord_to_check {
                    Some(next_coord_to_check) => match forest.trees.get(&next_coord_to_check) {
                        Some(hight_of_next_tree) if hight_of_next_tree > tree_to_visit_hight => {
                            // println!(
                            //     "{:?} {:?} visible from {:?} {:?}",
                            //     next_coord_to_check,
                            //     hight_of_next_tree,
                            //     tree_to_visit,
                            //     tree_to_visit_hight
                            // );
                            println!(
                                "looking at tree {:?} of hight {:?}",
                                next_coord_to_check, hight_of_next_tree
                            );
                            // visible.insert(tree_to_visit);
                            println!("adding {:?}", tree_to_visit);
                            if forest.trees.contains_key(&next_coord_to_check) {
                                visible.insert(next_coord_to_check);
                                to_visit.push_front((next_coord_to_check, direction));
                            }
                        }
                        // another higher tree might be behind
                        Some(hight_of_next_tree) if hight_of_next_tree == tree_to_visit_hight => {
                            println!(
                                "{:?} {:?} NOT visible from {:?} {:?}",
                                next_coord_to_check,
                                hight_of_next_tree,
                                tree_to_visit,
                                tree_to_visit_hight
                            );
                            if forest.trees.contains_key(&next_coord_to_check) {
                                to_visit.push_front((next_coord_to_check, direction));
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            _ => (),
        }
        println!("to_visit {:?}", to_visit);

        count += 1;
    }
    println!("visible {:?}", visible);
    return visible.len();
}

pub fn part1(input: &str) -> Result<usize, String> {
    println!("bonjour");
    let forest = parse_into_forest(input);
    println!("forest {:?}", forest);
    let res = visit_forest(forest);
    return Ok(res);
}

pub fn part2(input: &str) -> Result<u64, String> {
    return todo!();
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    fn input() -> String {
        return "30373
25512
65332
33549
35390"
            .to_string();
    }

    #[test]
    fn find_tree_visibiilty_should_return_for_sides() {
        let forest = parse_into_forest(&input());
        assert_eq!(
            default_visibility(&forest, (0, 0)),
            VisibleFrom {
                directions: HashSet::from([VisibilityDirections::Top, VisibilityDirections::Left])
            }
        );
        assert_eq!(
            default_visibility(&forest, (0, 4)),
            VisibleFrom {
                directions: HashSet::from([VisibilityDirections::Top, VisibilityDirections::Right])
            }
        );
        assert_eq!(
            default_visibility(&forest, (1, 0)),
            VisibleFrom {
                directions: HashSet::from([VisibilityDirections::Left])
            }
        );
        assert_eq!(
            default_visibility(&forest, (4, 0)),
            VisibleFrom {
                directions: HashSet::from([
                    VisibilityDirections::Left,
                    VisibilityDirections::Bottom
                ])
            }
        );
        assert_eq!(
            default_visibility(&forest, (4, 4)),
            VisibleFrom {
                directions: HashSet::from([
                    VisibilityDirections::Right,
                    VisibilityDirections::Bottom
                ])
            }
        )
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(&input()), Ok(21));
    }

    // #[test]
    // fn part1_result() {
    //     let input = read_file("resources/day8.txt");
    //     assert_eq!(part1(&input), Ok(429));
    // }

    fn read_file(file_name: &str) -> String {
        return fs::read_to_string(file_name).expect("Unable to read the file");
    }
}
