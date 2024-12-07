use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
struct Guard {
    position: (isize, isize),
    direction: DIRECTION,
}

impl Guard {
    fn next_position(&self) -> (isize, isize) {
        let delta: (isize, isize) = match self.direction {
            DIRECTION::Up => (0, -1),
            DIRECTION::Right => (1, 0),
            DIRECTION::Down => (0, 1),
            DIRECTION::Left => (-1, 0),
        };

        (self.position.0 + delta.0, self.position.1 + delta.1)
    }

    fn step(&mut self) {
        self.position = self.next_position();
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum DIRECTION {
    Up,
    Right,
    Down,
    Left,
}

impl DIRECTION {
    fn from(c: char) -> DIRECTION {
        match c {
            '>' => DIRECTION::Right,
            'v' => DIRECTION::Down,
            '<' => DIRECTION::Left,
            '^' => DIRECTION::Up,
            _ => panic!(),
        }
    }

    fn next(&self) -> DIRECTION {
        match self {
            DIRECTION::Up => DIRECTION::Right,
            DIRECTION::Right => DIRECTION::Down,
            DIRECTION::Down => DIRECTION::Left,
            DIRECTION::Left => DIRECTION::Up,
        }
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, Guard) {
    let mut guard = Guard {
        position: (0, 0),
        direction: DIRECTION::Left,
    };
    let mut grid: Vec<Vec<char>> = vec![];

    for (i, line) in input.lines().enumerate() {
        grid.push(line.trim().chars().collect());

        match line.find(|c| ['^', '>', 'v', '<'].contains(&c)) {
            None => (),
            Some(c) => {
                guard.position = (c as isize, i as isize);
                guard.direction = DIRECTION::from(line.chars().nth(c).unwrap());
                grid[i][c] = '.';
            }
        }
    }

    (grid, guard)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, mut guard) = parse_grid(input);

    loop {
        let (x, y) = guard.position;
        grid[y as usize][x as usize] = 'x';
        let (x_prime, y_prime) = guard.next_position();
        if x_prime < 0
            || x_prime >= grid[0].len() as isize
            || y_prime < 0
            || y_prime >= grid.len() as isize
        {
            break;
        }
        if grid[y_prime as usize][x_prime as usize] == '#' {
            guard.direction = guard.direction.next();
        }
        guard.step();
    }

    Some(grid.iter().flatten().filter(|&&c| c == 'x').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, mut guard) = parse_grid(input);

    let init_guard = guard.clone();

    loop {
        let (x, y) = guard.position;
        grid[y as usize][x as usize] = 'x';
        let (x_prime, y_prime) = guard.next_position();
        if x_prime < 0
            || x_prime >= grid[0].len() as isize
            || y_prime < 0
            || y_prime >= grid.len() as isize
        {
            break;
        }
        if grid[y_prime as usize][x_prime as usize] == '#' {
            guard.direction = guard.direction.next();
        }
        guard.step();
    }

    grid[init_guard.position.0 as usize][init_guard.position.1 as usize] = '.';

    let potential_positions: HashSet<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(y, list)| list.into_iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .flatten()
        .filter(|(_, _, c)| c == &'x')
        .map(|(x, y, _)| (x, y))
        .collect();

    let mut cum = 0;

    for (x, y) in potential_positions {
        let mut new_grid = grid.clone();
        new_grid[y][x] = '#';
        let mut guard = init_guard.clone();
        let mut previous_pose: HashSet<(isize, isize, DIRECTION)> = HashSet::new();

        loop {
            if previous_pose.contains(&(
                guard.position.0,
                guard.position.1,
                guard.direction.clone(),
            )) {
                cum += 1;
                break;
            }
            let (x, y) = guard.position;
            new_grid[y as usize][x as usize] = 'x';

            let (x_prime, y_prime) = guard.next_position();
            if x_prime < 0
                || x_prime >= new_grid[0].len() as isize
                || y_prime < 0
                || y_prime >= new_grid.len() as isize
            {
                break;
            }

            previous_pose.insert((guard.position.0, guard.position.1, guard.direction));

            if new_grid[y_prime as usize][x_prime as usize] == '#' {
                guard.direction = guard.direction.next();
                let (x_prime, y_prime) = guard.next_position();
                if !(x_prime < 0
                    || x_prime >= new_grid[0].len() as isize
                    || y_prime < 0
                    || y_prime >= new_grid.len() as isize
                    || new_grid[y_prime as usize][x_prime as usize] == '#') {
                        guard.direction = guard.direction.next();

                }
            }
            guard.step();
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
