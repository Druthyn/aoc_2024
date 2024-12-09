advent_of_code::solution!(9);

fn parse_disk_map(input: &str) -> Vec<Option<u64>> {
    let mut index = 0;
    let mut spacing_iter = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());
    let mut out = vec![];

    while let Some(len) = spacing_iter.next() {
        for _ in 0..len {
            out.push(Some(index));
        }

        if let Some(gap) = spacing_iter.next() {
            for _ in 0..gap {
                out.push(None);
            }
        }
        index += 1;
    }

    out
}

fn checksum(disk: &[Option<u64>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter(|(_, e)| e.is_some())
        .map(|(i, e)| e.unwrap() * i as u64)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = parse_disk_map(input);

    let mut head = 0;
    let mut tail = disk.len() - 1;

    while head != tail {
        if disk[head].is_some() {
            head += 1;
            continue;
        }
        while disk[tail].is_none() {
            tail -= 1;
        }
        disk.swap(head, tail);
        tail -= 1;
    }

    Some(checksum(&disk))
}

pub fn part_two(input: &str) -> Option<u64> {
    fn block_size(disk: &[Option<u64>], index: usize) -> usize {
        let mut count = 1;

        let mut head = index;
        while head < disk.len() - 1 && disk[head] == disk[head + 1] {
            head += 1;
            count += 1;
        }

        count
    }

    fn find_gaps(disk: &[Option<u64>]) -> Vec<(usize, usize)> {
        find_block(disk, false)
    }

    fn find_files(disk: &[Option<u64>]) -> Vec<(usize, usize)> {
        find_block(disk, true)
    }

    fn find_block(disk: &[Option<u64>], is_none: bool) -> Vec<(usize, usize)> {
        let mut head = 0;

        let mut out = vec![];

        while head < disk.len() {
            while head < disk.len() && disk[head].is_none() == is_none {
                head += 1;
            }
            let block_size = block_size(disk, head);
            out.push((head, block_size));

            head += block_size;
        }

        out
    }

    let mut disk = parse_disk_map(input);

    let mut files = find_files(&disk);
    files.reverse();

    for file in files {
        for gap in find_gaps(&disk) {
            if file.1 <= gap.1 && file.0 > gap.0 {
                for delta in 0..file.1 {
                    disk.swap(gap.0 + delta, file.0 + delta);
                }
                break;
            }
        }
    }

    Some(checksum(&disk))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn answer_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        let answer = advent_of_code::template::read_file_part("answers", DAY, 1)
            .parse()
            .unwrap();
        assert_eq!(result, Some(answer));
    }

    #[test]
    fn answer_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        let answer = advent_of_code::template::read_file_part("answers", DAY, 2)
            .parse()
            .unwrap();
        assert_eq!(result, Some(answer));
    }
}
