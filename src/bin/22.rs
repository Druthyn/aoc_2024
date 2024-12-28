use std::ops::{BitAnd, BitXor};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

    for _ in 0..2000 {
        numbers = numbers
            .iter()
            .map(|v| v.bitxor(v << 6).bitand(0xFFFFFF))
            .map(|v| v.bitxor(v >> 5).bitand(0xFFFFFF))
            .map(|v| v.bitxor(v << 11).bitand(0xFFFFFF))
            .collect();
    }

    Some(numbers.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    // let mut numbers: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
