use std::ops::BitAnd;

use advent_of_code::split_input_at_emptyline;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let keylocks = split_input_at_emptyline(input);

    let mut keys = vec![];
    let mut locks = vec![];

    for item in keylocks {
        let number = item
            .join("")
            .chars()
            .filter(|&c| c == '.' || c == '#')
            .map(|c| if c == '.' { 0 } else { 1 })
            .fold(0, |acc, digit| (acc << 1) + digit);

        if number.bitand(0b11111) == 0b11111 {
            locks.push(number);
        } else {
            keys.push(number);
        }
    }

    let mut cum = 0;

    for key in keys {
        for lock in &locks {
            if key.bitand(lock) == 0 {
                cum += 1;
            }
        }
    }
    Some(cum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
