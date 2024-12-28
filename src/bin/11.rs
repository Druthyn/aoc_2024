use std::vec;

advent_of_code::solution!(11);

fn blink_naive(stones: &[u64]) -> Vec<u64> {
    let mut out = vec![];
    for &stone in stones {
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        if stone == 0 {
            out.push(1);
        } else {
            // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of
            // the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
            // (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
            let mut stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let stone_2 = stone_str.split_off(stone_str.len() / 2);
                out.push(stone_str.parse().unwrap());
                out.push(stone_2.parse().unwrap());
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is
                // engraved on the new stone.
                out.push(stone * 2024)
            }
        }
    }
    out
}

// fn blink(_stones: &[u64]) -> Vec<u64> {
//     todo!()
// }

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    for _ in 0..25 {
        stones = blink_naive(&stones);
    }
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    for _ in 0..7 {
        stones = blink_naive(&stones);
    }
    Some(stones.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
