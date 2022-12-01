

pub fn dayone() -> u32 {
    println!("Hello, world!");
    return 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_should_return_1(){
        assert_eq!(dayone(), 1);
    }

    
}

