advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| s.split("   ").map(|s| s.parse::<u32>().unwrap()))
            .map(|mut e| e.next().unwrap().abs_diff(e.next().unwrap()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = input.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let left: Vec<u32> = parsed_input.clone().step_by(2).collect();
    let right: Vec<u32> = parsed_input.skip(1).step_by(2).collect();

    let mut right_buckets = HashMap::new();

    for v in right {
        right_buckets.insert(v, right_buckets.get(&v).unwrap_or(&0) + 1);
    }

    let mut sum = 0;
    for v in left {
        sum += right_buckets.get(&v).unwrap_or(&0) * v;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 31);
    }
}
