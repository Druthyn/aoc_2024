use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

// rules in a map of first -> things that have to be after it

// for p in pages {
//  if p in map {
//  assert intersection of that map and seen pages is 0
// }
//}

type RuleMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input(input: &str) -> (RuleMap, Vec<Vec<&str>>) {
    let mut rules: Vec<&str> = vec![];
    let mut pages: Vec<Vec<&str>> = vec![];

    let mut is_pages = false;

    for line in input.lines() {
        if line.is_empty() {
            is_pages = true;
            continue;
        }
        if is_pages {
            pages.push(line.split(',').collect());
        } else {
            rules.push(line);
        }
    }

    let mut rules_map: RuleMap = HashMap::new();

    for rule in rules {
        let mut sections = rule.split('|');
        let key = sections.next().unwrap();

        if !rules_map.contains_key(key) {
            rules_map.insert(key, HashSet::new());
        }
        rules_map
            .get_mut(key)
            .unwrap()
            .insert(sections.next().unwrap());
    }

    (rules_map, pages)
}

fn is_valid_pages(rules_map: &RuleMap, pages: &[&str]) -> bool {
    let mut seen_pages = HashSet::new();
    for &page in pages {
        let map = match rules_map.get(page) {
            Some(h) => h,
            None => {
                seen_pages.insert(page);
                continue;
            }
        };
        if !map.is_disjoint(&seen_pages) {
            return false;
        }

        seen_pages.insert(page);
    }
    true
}

fn order_pages<'a>(rules_map: &'a RuleMap, pages: &[&'a str]) -> Vec<&'a str> {
    let mut new_pages = vec![];

    let mut page_set: HashSet<&str> = HashSet::new();
    for &page in pages {
        page_set.insert(page);
    }

    // for page in pages
    //      want to find the page that can be inserted without breaking any of the other pages rules
    //      a page wont break its own rule, and based on the problem asked we can assert there is a unique ordering
    //      so find the rules for our pages, and find the item that isn't in those rules, then repeat till no more pages

    while page_set.len() > 1 {
        let mut page_values = HashSet::new();
        for page in &page_set {
            match rules_map.get(page) {
                Some(set) => {
                    set.clone().into_iter().for_each(|s| {
                        page_values.insert(s);
                    });
                }
                None => continue,
            }
        }
        let page = **page_set
            .difference(&page_values)
            .collect::<Vec<_>>()
            .first()
            .unwrap();

        page_set.remove(page);
        new_pages.push(page);
    }
    new_pages.push(page_set.drain().next().unwrap());

    new_pages
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_map, pages_list) = parse_input(input);

    let mut valid_pages = vec![];
    for pages in pages_list {
        if is_valid_pages(&rules_map, &pages) {
            valid_pages.push(pages);
        }
    }

    let mut cum = 0;
    for pages in valid_pages {
        cum += pages[(pages.len() - 1) / 2].parse::<u32>().unwrap();
    }
    Some(cum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_map, pages_list) = parse_input(input);

    let mut new_orders = vec![];
    for pages in pages_list {
        if !is_valid_pages(&rules_map, &pages) {
            new_orders.push(order_pages(&rules_map, &pages));
        }
    }

    let mut cum = 0;
    for pages in new_orders {
        cum += pages[(pages.len() - 1) / 2].parse::<u32>().unwrap();
    }
    Some(cum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
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
