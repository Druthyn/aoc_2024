use std::collections::HashSet;

use advent_of_code::get_neighbours;

advent_of_code::solution!(10);

fn count_trails_bfs(x: usize, y: usize, grid: &[Vec<u32>]) -> usize {
    let mut search_stack = vec![(x, y)];
    let mut peak = 0;
    let mut search_set = HashSet::new();
    while peak != 9 && !search_stack.is_empty() {
        for (x, y) in search_stack.drain(..) {
            get_neighbours(x, y, grid)
                .into_iter()
                .flatten()
                .for_each(|v| {
                    search_set.insert(v);
                });
        }

        for (x, y) in search_set.drain() {
            if grid[y][x] == peak + 1 {
                search_stack.push((x, y));
            }
        }

        peak += 1;
    }

    search_stack.len()
}

fn count_trails_dfs(x: usize, y: usize, grid: &[Vec<u32>]) -> usize {
    let mut paths = 0;

    for (nx, ny) in get_neighbours(x, y, grid).into_iter().flatten() {
        if grid[ny][nx] == grid[y][x] + 1 {
            paths += count_trails_dfs(nx, ny, grid);
        } else if grid[y][x] == 9 {
            return 1;
        }
    }
    paths
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.lines() {
        let line_vec: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(line_vec);
    }

    let mut sum = 0;
    for x in 0..grid[0].len() {
        for (y, _) in grid.iter().enumerate() {
            if grid[y][x] == 0 {
                sum += count_trails_bfs(x, y, &grid);
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = vec![];
    for line in input.lines() {
        let line_vec: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(line_vec);
    }

    let mut sum = 0;
    for x in 0..grid[0].len() {
        for (y, _) in grid.iter().enumerate() {
            if grid[y][x] == 0 {
                sum += count_trails_dfs(x, y, &grid);
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
