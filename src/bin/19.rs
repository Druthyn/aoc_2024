use advent_of_code::split_input_at_emptyline;
use regex::Regex;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let x = split_input_at_emptyline(input);

    let search = "^(".to_string()
        + &x[0][0]
            .chars()
            .filter(|&c| c != ' ')
            .map(|c| if c == ',' { '|' } else { c })
            .collect::<String>()
        + ")+$";

    let re = Regex::new(&search).ok()?;

    let mut cum = 0;

    for towel in &x[1] {
        if re.is_match(towel) {
            cum += 1;
        }
    }

    Some(cum)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
