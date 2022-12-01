use std::fs;
use std::str;


fn read_file(fileName: &str)-> String {
    return fs::read_to_string(fileName)
        .expect("Unable to read the file");
}

fn sumCaloriesOfSingleElf(rawData: &str) -> u32{
    return rawData
    .split('\n')
    .map(|calories| calories.parse::<u32>().unwrap())
    .sum::<u32>();
} 

#[warn(dead_code)]
pub fn dayone() -> Option<u32> {
    return 
        read_file("resources/day1.txt")
        .split("\n\n")
        .map(sumCaloriesOfSingleElf)
        .max();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_should_return_1(){
        assert_eq!(dayone(), Some(67633));
    }

    
}

