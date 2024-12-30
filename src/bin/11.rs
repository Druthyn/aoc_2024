use std::{collections::HashMap, vec};

advent_of_code::solution!(11);

#[allow(dead_code)]
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

fn blink(stones: &[u64], iterations: usize) -> u64 {
    fn count(stone: u64, steps: usize, memos: &mut HashMap<(u64, usize), u64>) -> u64 {
        if memos.contains_key(&(stone, steps)) {
            return *memos.get(&(stone, steps)).unwrap();
        }
        if steps == 0 {
            return 1;
        }

        let res;

        if stone == 0 {
            res = count(1, steps - 1, memos);
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let stone_front = stone / 10_u64.pow((stone.ilog10() + 1) / 2);
            let stone_back = stone - stone_front * 10_u64.pow((stone.ilog10() + 1) / 2);
            res = count(stone_front, steps - 1, memos) + count(stone_back, steps - 1, memos);
        } else {
            res = count(stone * 2024, steps - 1, memos);
        }
        memos.insert((stone, steps), res);
        res
    }

    let mut memos: HashMap<(u64, usize), u64> = HashMap::new();

    let mut sum = 0;
    for stone in stones {
        sum += count(*stone, iterations, &mut memos);
    }

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    Some(blink(&stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    Some(blink(&stones, 75))
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
