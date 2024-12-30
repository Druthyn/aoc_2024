use std::{collections::HashMap, iter::successors};

advent_of_code::solution!(7);

fn operations_builder(
    n: usize,
    is_part2: bool,
    memos: &mut HashMap<(usize, bool), Vec<u64>>,
) -> Vec<u64> {
    if memos.contains_key(&(n, is_part2)) {
        return memos.get(&(n, is_part2)).unwrap().clone();
    }

    if n == 0 {
        return if is_part2 { vec![123] } else { vec![12] };
    }

    let previous = operations_builder(n - 1, is_part2, memos);
    let mut out = vec![];
    for list in previous {
        out.push(list * 10 + 1);
        out.push(list * 10 + 2);
        if is_part2 {
            out.push(list * 10 + 3);
        }
    }

    memos.insert((n, is_part2), out.clone());
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

pub fn solve(input: &str, is_part2: bool) -> Option<u64> {
    let mut cum = 0;

    let mut memos = HashMap::new();

    for line in input.lines() {
        let (res, components) = parse_line(line);
        let operations = operations_builder(components.len() - 1, is_part2, &mut memos);
        for ops in operations {
            let mut numbers = components.clone().into_iter();
            let mut acc = numbers.next().unwrap();

            let x = successors(Some((0, ops)), |(_, op)| {
                if *op == 0 {
                None
            } else {
                Some((
                    op % 10,
                    op / 10
                ))
            }}).skip(1).map(|(n, _)| n);

            for (num, op) in numbers.zip(x) {
                match op {
                    1 => acc += num,
                    2 => acc *= num,
                    3 => {
                        acc *= 10_u64.pow(num.ilog10() + 1);
                        acc += num;
                    }
                    _ => panic!("{}", ops),

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

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
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
