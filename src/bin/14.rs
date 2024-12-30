use advent_of_code::grid::Grid;

advent_of_code::solution!(14);

fn uniqueness(map: &Grid<u32>) -> usize {
    (map.len() * map[0].len()) - map.iter().flat_map(|l| l.iter()).filter(|v| **v == 0).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let is_testing = input.lines().next().unwrap() == "p=0,4 v=3,-3";

    let dimensions = if is_testing { (11, 7) } else { (101, 103) };

    let mut map = Grid::new(vec![vec![0; dimensions.0]; dimensions.1]);

    for robot in input.lines() {
        let (pos_str, vel_str) = robot.split_once(' ')?;
        let mut pos_iter = pos_str[2..].split(',').map(|s| s.parse::<i32>());
        let pos = (pos_iter.next()?.ok()?, pos_iter.next()?.ok()?);
        let mut vel_iter = vel_str[2..].split(',').map(|s| s.parse::<i32>());
        let vel = (vel_iter.next()?.ok()?, vel_iter.next()?.ok()?);

        let mut end_x = (pos.0 + 100 * vel.0) % dimensions.0 as i32;
        if end_x < 0 {
            end_x += dimensions.0 as i32;
        }
        let mut end_y = (pos.1 + 100 * vel.1) % dimensions.1 as i32;
        if end_y < 0 {
            end_y += dimensions.1 as i32;
        }

        let end = (end_x as usize, end_y as usize);

        map.set(end, map.get(end)? + 1).unwrap();
    }

    let tl: u32 = map[0..(dimensions.1 - 1) / 2]
        .iter()
        .flat_map(|l| l[0..(dimensions.0 - 1) / 2].iter())
        .sum();
    let bl: u32 = map[(dimensions.1 + 1) / 2..]
        .iter()
        .flat_map(|l| l[0..(dimensions.0 - 1) / 2].iter())
        .sum();
    let tr: u32 = map[0..(dimensions.1 - 1) / 2]
        .iter()
        .flat_map(|l| l[(dimensions.0 + 1) / 2..].iter())
        .sum();
    let br: u32 = map[(dimensions.1 + 1) / 2..]
        .iter()
        .flat_map(|l| l[(dimensions.0 + 1) / 2..].iter())
        .sum();

    Some(tl * tr * bl * br)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dimensions =  (101, 103);

    for n in 6377..99999999 {
        let mut map = Grid::new(vec![vec![0; dimensions.0]; dimensions.1]);
        for robot in input.lines() {
            let (pos_str, vel_str) = robot.split_once(' ')?;
            let mut pos_iter = pos_str[2..].split(',').map(|s| s.parse::<i32>());
            let pos = (pos_iter.next()?.ok()?, pos_iter.next()?.ok()?);
            let mut vel_iter = vel_str[2..].split(',').map(|s| s.parse::<i32>());
            let vel = (vel_iter.next()?.ok()?, vel_iter.next()?.ok()?);

            let mut end_x = (pos.0 + n * vel.0) % dimensions.0 as i32;
            if end_x < 0 {
                end_x += dimensions.0 as i32;
            }
            let mut end_y = (pos.1 + n * vel.1) % dimensions.1 as i32;
            if end_y < 0 {
                end_y += dimensions.1 as i32;
            }

            let end = (end_x as usize, end_y as usize);

            map.set(end, map.get(end)? + 1).unwrap();
        }


        if uniqueness(&map) >= 500 {
            println!("iterations: {}", n);
            println!("{}", map);
            return Some(n as u32)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
