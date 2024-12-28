use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut garden = vec![];
    for line in input.lines() {
        garden.push(line.chars().collect::<Vec<char>>());
    }

    let symbol_set: HashSet<&char> = HashSet::from_iter(garden.iter().flat_map(|l| l.iter()));

    for _ in symbol_set {

    }

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
