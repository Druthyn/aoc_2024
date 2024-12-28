use advent_of_code::grid::Grid;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let map = Grid::new(input.lines().map(|l| l.chars().collect()).collect());
    let _start = (1_usize, map.len() - 2);
    let _end = (map[0].len() - 2, 1_usize);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }
}
