advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    Some(
        re.find_iter(input)
            .map(|x| {
                x.as_str()[4..x.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .product::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)()()|don't\(\)()()").unwrap();

    let mut flag: bool = true;
    let mut sum = 0;
    for x in re.find_iter(input).map(|x| x.as_str()) {
        match &x[0..3] {
            "mul" => {
                if flag {
                    sum += x[4..x.len() - 1]
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .product::<u32>();
                }
            }
            "do(" => flag = true,
            "don" => flag = false,
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
