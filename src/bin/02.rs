use std::ops::Deref;

advent_of_code::solution!(2);

struct U32Str<'a>(&'a str);

impl<'a> Deref for U32Str<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<U32Str<'_>> for Vec<u32> {
    fn from(value: U32Str) -> Self {
        value
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

fn is_report_safe(report: &mut [u32]) -> bool {
    // Reverse list if needed for sorted check
    if report[0] > report[1] {
        report.reverse();
    }

    // Reports must be increasing (or decreasing but reversed)
    if !report.is_sorted() {
        return false;
    }

    for i in 0..report.len() - 1 {
        // Adjacent levels must differ by at least one, and at most three
        if !(1..=3).contains(&(report[i + 1] - report[i])) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let mut list: Vec<u32> = U32Str(line).into();
        count += is_report_safe(&mut list) as u32;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let mut list: Vec<u32> = U32Str(line).into();

        match is_report_safe(&mut list) {
            true => count += 1,
            false => {
                for i in 0..list.len() {
                    let (_, mut sublist): (Vec<usize>, Vec<u32>) = list
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index != i)
                        .unzip();
                    if is_report_safe(&mut sublist) {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn answer_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        let answer = advent_of_code::template::read_file_part("answers", DAY, 1)
            .parse()
            .unwrap();
        assert_eq!(result, Some(answer));
    }

    #[test]
    fn answer_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        let answer = advent_of_code::template::read_file_part("answers", DAY, 2)
            .parse()
            .unwrap();
        assert_eq!(result, Some(answer));
    }
}
