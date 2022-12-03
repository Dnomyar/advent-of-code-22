use std::{fs, collections::HashSet};


fn read_file(file_name: &str)-> String {
    return fs::read_to_string(file_name)
        .expect("Unable to read the file");
}

#[derive(Debug)]
struct Rucksack{
    compatiment1: HashSet<char>,
    compatiment2: HashSet<char>,
}

impl TryFrom<&str> for Rucksack {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        println!("FROM {}", value);
        let size = value.len();
        let (left, right) = value.split_at(size/2);
        let rucksack = Rucksack {
            compatiment1: HashSet::from_iter(left.chars()),
            compatiment2: HashSet::from_iter(right.chars())
        };
        return Ok(rucksack);
    }
}

struct Priority {
    priority: u32
}

impl TryFrom<char> for Priority {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        println!("value={0}", value);
        let n = value as u32;
        if n >= 'a' as u32 && n <= 'z'  as u32 {
            let p = n - 'a' as u32 + 1;
            println!("p={0}", p);
            return Ok(Priority{ priority: p });
        } else if n >= 'A'  as u32 && n <= 'Z' as u32  {
            let p = n - 'A' as u32 + 27;
            println!("p={0}", p);
            return Ok(Priority{ priority: p});
        } else {
            return Err(format!("Unable to find priority from ${0}", value))
        };
    }
}



fn sumPriorities(rucksacks: Vec<Rucksack>) -> Result<u32, String> {
    let priorities: Result<Vec<Priority>, String> = rucksacks.into_iter().map(|rucksack| {
        let intersection: Option<&char> = rucksack.compatiment1.intersection(&rucksack.compatiment2).last();
        return match intersection  {
            None => Err("no intersection".to_owned()),
            Some(&char) => 
                Priority::try_from(char)
        }
    }).collect();

    return priorities.and_then(|priorities| 
        Ok(priorities.into_iter().map(|p|p.priority).sum())
    );
}

pub fn part1(input: &str) -> Result<u32, String> {
    let rucksacksResult: Result<Vec<Rucksack>, String> = 
        input
            .lines()
            .map(|line| line.trim())
            .map(Rucksack::try_from)
            .collect();

    return rucksacksResult.and_then(sumPriorities);
}

pub fn part2() -> Option<u32> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_example(){
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part1(&input), Ok(157));
    }

    #[test]
    fn part1_resutl(){
        let input = read_file("resources/day3.txt");
        assert_eq!(part1(&input), Err("".to_string()));
    }

    
}
