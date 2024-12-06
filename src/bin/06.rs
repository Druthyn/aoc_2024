advent_of_code::solution!(6);

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
    let (mut _grid, _guard) = parse_grid(input);

    None
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
