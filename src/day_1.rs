use std::fs;

use std::str;

use priority_queue::PriorityQueue;

fn read_file(fileName: &str) -> String {
    return fs::read_to_string(fileName).expect("Unable to read the file");
}

fn parse_elves_group(rawElf: &str) -> impl Iterator<Item = u32> + '_ {
    return rawElf
        .split('\n')
        .map(|calories| calories.parse::<u32>().unwrap());
}

fn parse_elves_groups(rawData: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
    return rawData.split("\n\n").map(parse_elves_group);
}

#[warn(dead_code)]
pub fn dayone_part1() -> Option<u32> {
    let file = read_file("resources/day1.txt");
    return parse_elves_groups(&file)
        .map(|calory_group| calory_group.sum())
        .max();
}

pub fn dayone_part2() -> Option<u32> {
    let file = read_file("resources/day1.txt");
    let numberOfElvesToSum: usize = 3;
    let mut pq: PriorityQueue<u32, ()> = PriorityQueue::new();

    let group_summed = parse_elves_groups(&file).map(|calory_group| calory_group.sum::<u32>());

    group_summed.for_each(|calory_sum| {
        let should_update_max = if let Some((previous_max, _)) = pq.peek() {
            previous_max < &calory_sum
        } else {
            true
        };

        if should_update_max {
            if pq.len() >= numberOfElvesToSum {
                pq.pop();
            }

            pq.push(calory_sum, ());
        }
    });

    return Some(pq.into_sorted_iter().map(|e| e.0).sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dayone_part1_test() {
        assert_eq!(dayone_part1(), Some(67633));
    }

    #[test]
    fn dayone_part2_test() {
        assert_eq!(dayone_part2(), Some(199628));
    }
}
