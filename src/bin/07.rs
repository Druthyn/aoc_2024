advent_of_code::solution!(7);

fn operations_builder(n: usize, is_part2: bool) -> Vec<Vec<char>> {
    fn operation_builder_helper(input: Vec<Vec<char>>, is_part2: bool) -> Vec<Vec<char>> {
        let mut out = vec![];
        for mut list in input {
            let mut list2 = list.clone();
            list2.push('*');
            out.push(list2);
            if is_part2 {
                let mut list3 = list.clone();
                list3.push('|');
                out.push(list3);
            }
            list.push('+');
            out.push(list)
        }
        out
    }

    let mut out = vec![Vec::new()];
    for _ in 0..n {
        out = operation_builder_helper(out, is_part2)
    }
    out
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts_iter = line.split(' ');

    let res = parts_iter
        .next()
        .unwrap()
        .trim_matches(':')
        .parse::<u64>()
        .unwrap();

    let components = Vec::from_iter(parts_iter.map(|e| e.parse::<u64>().unwrap()));

    (res, components)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cum = 0;

    for line in input.lines() {
        let (res, components) = parse_line(line);
        let operations = operations_builder(components.len() - 1, false);
        for op in operations {
            let mut numbers = components.clone().into_iter();
            let mut acc = numbers.next().unwrap();

            for (num, op) in numbers.zip(op.into_iter()) {
                if op == '+' {
                    acc += num;
                }
                if op == '*' {
                    acc *= num;
                }
            }

            if res == acc {
                cum += res;
                break;
            }
        }
    }
    Some(cum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cum = 0;

    for line in input.lines() {
        let (res, components) = parse_line(line);
        let operations = operations_builder(components.len() - 1, true);
        for op in operations {
            let mut numbers = components.clone().into_iter();
            let mut acc = numbers.next().unwrap();

            for (num, op) in numbers.zip(op.into_iter()) {
                if op == '+' {
                    acc += num;
                }
                if op == '*' {
                    acc *= num;
                }
                if op == '|' {
                    acc *= 10_u64.pow(num.to_string().len() as u32);
                    acc += num;
                }
            }

            if res == acc {
                cum += res;
                break;
            }
        }
    }
    Some(cum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
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
