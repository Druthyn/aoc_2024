advent_of_code::solution!(4);

fn find_xmas_substring(input: &str) -> u32 {
    if input.len() < 4 {
        return 0;
    }
    let mut count = 0;
    let max = input.len() - 3;
    for sub in ["XMAS", "SAMX"] {
        for i in 0..max {
            if input[i..(i + 4)] == *sub {
                count += 1;
            }
        }
    }
    count
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    fn get_diagnonals(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut lines = vec![];

        for offset in 0.. {
            let mut line: Vec<char> = vec![];
            for i in 0.. {
                match grid.get(i + offset) {
                    Some(&ref l) => match l.get(i) {
                        Some(&c) => line.push(c),
                        None => break,
                    },
                    None => break,
                }
            }
            if line.len() == 0 {
                break;
            }
            lines.push(line);
        }

        for offset in 1.. {
            let mut line: Vec<char> = vec![];
            for i in 0.. {
                match grid.get(i) {
                    Some(&ref l) => match l.get(i + offset) {
                        Some(&c) => line.push(c),
                        None => break,
                    },
                    None => break,
                }
            }
            if line.len() == 0 {
                break;
            }
            lines.push(line);
        }

        lines
    }
    let mut grid: Vec<Vec<char>> = vec![];

    let mut cum = 0;

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for line in &grid {
        cum += find_xmas_substring(&String::from_iter(line.iter()));
    }

    grid = transpose2(grid);
    for line in &grid {
        cum += find_xmas_substring(&String::from_iter(line.iter()));
    }

    let mut diags: Vec<Vec<char>> = get_diagnonals(&grid);
    grid.reverse();
    diags.append(&mut get_diagnonals(&grid));
    for line in diags {
        cum += find_xmas_substring(&String::from_iter(line.iter()));
    }

    Some(cum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];

    let search_pair = ['M', 'S'];

    let mut cum = 0;

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] != 'A' {
                continue;
            }
            if !(search_pair.contains(&grid[y - 1][x - 1])
                && search_pair.contains(&grid[y + 1][x + 1])
                && grid[y - 1][x - 1] != grid[y + 1][x + 1])
            {
                continue;
            }
            if !(search_pair.contains(&grid[y - 1][x + 1])
                && search_pair.contains(&grid[y + 1][x - 1])
                && grid[y - 1][x + 1] != grid[y + 1][x - 1])
            {
                continue;
            }
            cum += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
