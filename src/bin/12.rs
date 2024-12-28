use std::collections::HashSet;

use advent_of_code::grid::Grid;

advent_of_code::solution!(12);

fn find_plots(garden: Grid<char>) -> Vec<HashSet<(usize, usize)>> {
    let mut coords: HashSet<(usize, usize)> = garden
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
        .collect();

    let mut plots = vec![];

    loop {
        if coords.is_empty() {
            break;
        }
        let &coord = coords.iter().next().unwrap();
        let plant = garden.get(coord).unwrap();
        
        let mut visited = HashSet::new();
        visited.insert(coord);
        let mut queue = vec![coord];

        while let Some(q) = queue.pop() {
            for neighbour in garden.get_neighbours_4(q).iter().filter(|(t, _)|t == plant).map(|(_, p)| p) {
                if !visited.contains(neighbour) {
                    queue.push(*neighbour);
                    visited.insert(*neighbour);
                }
            }
        }
        
        plots.push(visited.clone());
        for point in visited.iter() {
            coords.remove(point);
        }
    }
    plots
}

fn fence_perimeter(plot: HashSet<(usize, usize)>) -> u32 {
    fn calc_neighbours(point: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut out = HashSet::new();
        
        if point.0 != 0 {
            out.insert((point.0 - 1, point.1));
        }
        out.insert((point.0 + 1, point.1));

        if point.1 != 0 {
            out.insert((point.0, point.1 - 1));
        }
        out.insert((point.0, point.1 + 1));

        out
    }

    match &plot.len() {
        1 => 4,
        2 => 6,
        _ => {
            let mut perimeter = 0;
            for &coord in &plot {
                perimeter += 4 - plot.intersection(&calc_neighbours(coord)).count();
            }   
            perimeter as u32
        }
    }
}

fn side_count(_plot: HashSet<(usize, usize)>) -> u32 {
    // for plant in plot {
    //     let xs = if plant.0 == 0 {
    //         [plant.0..(plant.0 + 1)]
    //     } else {
    //         [(plant.0 - 1)..(plant.0 + 1)]
    //     };
    //     let ys = if plant.1 == 0 {
    //         [plant.1..(plant.1 + 1)]
    //     } else {
    //         [(plant.1 - 1)..(plant.1 + 1)]
    //     };
    // }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let garden = Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    let plots = find_plots(garden);

    let mut sum = 0;

    for plot in plots {
        sum += plot.len() as u32 * fence_perimeter(plot);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let garden = Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    let plots = find_plots(garden);

    let mut sum = 0;

    for plot in plots {
        sum += plot.len() as u32 * side_count(plot);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
