use core::panic;

use advent_of_code::{grid::Grid, split_input_at_emptyline};

advent_of_code::solution!(15);

fn gps(map: &[Vec<char>]) -> u32 {
    map.iter()
        .enumerate()
        .flat_map(|(y, list)| list.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|(_, _, c)| c == &'O')
        .map(|(x, y, _)| x + 100 * y)
        .sum::<usize>() as u32
}

fn gps2(map: &[Vec<char>]) -> u32 {
    map.iter()
        .enumerate()
        .flat_map(|(y, list)| list.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|(_, _, c)| c == &'[')
        .map(|(x, y, _)| x + 100 * y)
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_moves = split_input_at_emptyline(input);

    let mut map = Grid::new(map_moves[0].iter().map(|l| l.chars().collect()).collect());

    // println!("{}", map);

    let mut position = map
        .iter()
        .enumerate()
        .flat_map(|(y, list)| list.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .find(|(_, _, c)| c == &'@')
        .map(|(x, y, _)| (x, y))?;

    for m in map_moves[1].iter().flat_map(|l| l.chars()) {
        let new_position = match m {
            '<' => (position.0 - 1, position.1),
            '^' => (position.0, position.1 - 1),
            '>' => (position.0 + 1, position.1),
            'v' => (position.0, position.1 + 1),
            _ => panic!(),
        };

        match map.get(new_position) {
            Some('.') => {
                map.swap(position, new_position);
                position = new_position;
            }
            Some('#') => (),
            Some('O') => {
                let mut test_position = new_position;
                loop {
                    test_position = match m {
                        '<' => (test_position.0 - 1, test_position.1),
                        '^' => (test_position.0, test_position.1 - 1),
                        '>' => (test_position.0 + 1, test_position.1),
                        'v' => (test_position.0, test_position.1 + 1),
                        _ => panic!(),
                    };
                    match map.get(test_position) {
                        Some('.') => {
                            map.swap(position, test_position);
                            map.swap(test_position, new_position);
                            position = new_position;
                            break;
                        }
                        Some('O') => (),
                        Some('#') => break,
                        Some(_) | None => panic!(),
                    }
                }
            }
            Some(_) | None => panic!(),
        }
        // println!("Move: {m}\n");
        // println!("{}", map);
    }

    Some(gps(&map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_moves = split_input_at_emptyline(input);

    let big_map = map_moves[0]
        .iter()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut map = Grid::new(big_map);

    println!("{map}");

    let mut position = map
        .iter()
        .enumerate()
        .flat_map(|(y, list)| list.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .find(|(_, _, c)| c == &'@')
        .map(|(x, y, _)| (x, y))?;

    for m in map_moves[1].iter().flat_map(|l| l.chars()) {
        let new_position = match m {
            '<' => (position.0 - 1, position.1),
            '^' => (position.0, position.1 - 1),
            '>' => (position.0 + 1, position.1),
            'v' => (position.0, position.1 + 1),
            _ => panic!(),
        };

        match map.get(new_position) {
            Some('.') => {
                map.swap(position, new_position);
                position = new_position;
            }
            Some('#') => (),
            Some('[') | Some(']') => {
                let mut test_position = new_position;
                loop {
                    match m {
                        '<' | '>' => {
                            test_position = match m {
                                '<' => (test_position.0 - 1, test_position.1),
                                '>' => (test_position.0 + 1, test_position.1),
                                _ => panic!(),
                            };
                            match map.get(test_position) {
                                Some('.') => {
                                    while test_position.0 != new_position.0 {
                                        let inter_position = match m {
                                            '<' => (test_position.0 + 1, test_position.1),
                                            '>' => (test_position.0 - 1, test_position.1),
                                            _ => panic!(),
                                        };
                                        map.swap(test_position, inter_position);
                                        test_position = inter_position;
                                    }
                                    map.swap(position, test_position);
                                    map.swap(test_position, new_position);
                                    position = new_position;
                                    break;
                                }
                                Some('[') | Some(']') => (),
                                Some('#') => break,
                                Some(_) | None => panic!(),
                            }
                        }
                        'v' | '^' => {}
                        _ => panic!(),
                    }
                }
            }
            Some(_) | None => panic!(),
        }
        // println!("Move: {m}\n");
        // println!("{}", map);
    }

    Some(gps2(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
