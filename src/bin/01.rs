advent_of_code::solution!(1);

use std::collections::HashMap;

use advent_of_code as aoc;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = aoc::split_file_into_int_lists(input);
    left.sort();
    right.sort();

    Some(left.into_iter()
                .zip(right)
                .map(|(l, r)| (l).abs_diff(r))
                .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = aoc::split_file_into_int_lists(input);
    let mut right_buckets = HashMap::new();

    for x in right {
        right_buckets.insert(x, right_buckets.get(&x).unwrap_or(&0) + 1);
    }

    let mut sum = 0;
    for x in left {
        sum += right_buckets.get(&x).unwrap_or(&0) * x;
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
