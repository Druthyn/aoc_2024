advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    Some(
        re.find_iter(input)
            .map(|reg_match| {
                reg_match.as_str()[4..reg_match.len() - 1]
                    .split(',')
                    .map(|int_str| int_str.parse::<u32>().unwrap())
                    .product::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)()()|don't\(\)()()").unwrap();

    let mut is_mul_active = true;
    let mut sum = 0;
    for reg_match in re.find_iter(input).map(|reg_match| reg_match.as_str()) {
        match &reg_match[0..3] {
            "mul" => {
                if is_mul_active {
                    sum += reg_match[4..reg_match.len() - 1]
                        .split(',')
                        .map(|int_str| int_str.parse::<u32>().unwrap())
                        .product::<u32>();
                }
            }
            "do(" => is_mul_active = true,
            "don" => is_mul_active = false,
            _ => panic!(),
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
