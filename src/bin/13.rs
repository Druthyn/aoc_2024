use advent_of_code::{lcm, split_input_at_emptyline};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let machines = split_input_at_emptyline(input);
    for machine in machines {
        let ax: i64 = machine[0][11..].split(',').next()?.parse().unwrap();
        let ay: i64 = machine[0][11..].split(',').nth(1).unwrap()[2..]
            .parse()
            .unwrap();

        let bx: i64 = machine[1][11..].split(',').next()?.parse().unwrap();
        let by: i64 = machine[1][11..].split(',').nth(1).unwrap()[2..]
            .parse()
            .unwrap();

        let x: i64 = machine[2][9..].split(',').next()?.parse().unwrap();
        let y: i64 = machine[2][9..].split(',').nth(1).unwrap()[3..]
            .parse()
            .unwrap();

        let k: i64 = lcm(ax, ay);

        let num = (x * k / ax) - (y * k / ay);
        let den = (bx * k / ax) - (by * k / ay);

        if num.rem_euclid(den) == 0 {
            let b = num / den;
            let a = (y - (by * b)) / ay;

            sum += 3 * a as u32 + b as u32;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let machines = split_input_at_emptyline(input);
    for machine in machines {
        let ax: i64 = machine[0][11..].split(',').next()?.parse().unwrap();
        let ay: i64 = machine[0][11..].split(',').nth(1)?[2..].parse().unwrap();

        let bx: i64 = machine[1][11..].split(',').next()?.parse().unwrap();
        let by: i64 = machine[1][11..].split(',').nth(1)?[2..].parse().unwrap();

        let x: i64 = machine[2][9..].split(',').next()?.parse::<i64>().unwrap() + 10000000000000;
        let y: i64 = machine[2][9..].split(',').nth(1)?[3..]
            .parse::<i64>()
            .unwrap()
            + 10000000000000;

        let k: i64 = lcm(ax, ay);

        let num = (x * k / ax) - (y * k / ay);
        let den = (bx * k / ax) - (by * k / ay);

        if num.rem_euclid(den) == 0 {
            let b = num / den;
            let a = (y - (by * b)) / ay;

            sum += 3 * a as u64 + b as u64;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
