use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type AntennaMap = HashMap<char, HashSet<Position>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position{x: x as isize, y: y as isize }
    }
}

impl std::ops::Deref for Position {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

fn parse_input(input: &str) -> (AntennaMap, (usize, usize)) {
    let mut out = HashMap::new();
    let mut dims = (0, 0);
    dims.1 = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        if dims.0 == 0 {
            dims.0 = line.chars().count();
        }
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                out.entry(c).or_insert_with(HashSet::new).insert(Position::new(x, y));
            });

    }
    (out, dims)
}

fn get_antinodes_part_1(a: &Position, b: &Position, limit: &(usize, usize)) -> Option<Vec<Position>> {
    fn is_valid(pos: Position, limit: &(usize, usize)) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x >= limit.0 as isize || pos.y >= limit.1 as isize)
    }
    let delta = *a - *b;
    let anti_a = *a + delta;
    let anti_b = *b - delta;

    let mut out = vec!{};
    if is_valid(anti_a, limit) {
        out.push(anti_a);
    }

    if is_valid(anti_b, limit) {
        out.push(anti_b);
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn get_antinodes_part_2(a: &Position, b: &Position, limit: &(usize, usize)) -> Option<Vec<Position>> {
    fn is_valid(pos: Position, limit: &(usize, usize)) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x >= limit.0 as isize || pos.y >= limit.1 as isize)
    }
    let delta = *a - *b;
    let mut anti_a = *a + delta;
    let mut anti_b = *b - delta;

    let mut out = vec!{};
    while is_valid(anti_a, limit) {
        out.push(anti_a);
        anti_a = anti_a + delta;
    }

    while is_valid(anti_b, limit) {
        out.push(anti_b);
        anti_b = anti_b + delta;
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (stations, limits) = parse_input(input);

    let mut antinodes = HashSet::new();

    for station_type in stations.keys() {
        for station1 in stations.get(station_type).unwrap() {
            for station2 in stations.get(station_type).unwrap() {
                if station1 != station2 {
                    if let Some(v) =  get_antinodes_part_1(station1, station2, &limits) {
                        for antinode in v {
                            antinodes.insert(antinode);
                        }
                    }

                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (stations, limits) = parse_input(input);

    let mut antinodes = HashSet::new();

    for station_type in stations.keys() {
        for station1 in stations.get(station_type).unwrap() {
            antinodes.insert(*station1);
            for station2 in stations.get(station_type).unwrap() {
                if station1 != station2 {
                    if let Some(v) =  get_antinodes_part_2(station1, station2, &limits) {
                        for antinode in v {
                            antinodes.insert(antinode);
                        }
                    }

                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
