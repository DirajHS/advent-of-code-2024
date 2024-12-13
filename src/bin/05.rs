use std::collections::HashMap;

advent_of_code::solution!(5);

fn process(input: &str, check_for_correct_order: bool) -> Option<u32> {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut process_updates = false;
    let result: u32 = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                process_updates = true;
                None
            } else if process_updates {
                let page_list: Vec<u32> = line
                    .split(',')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();

                let mut processed_pages: Vec<u32> = Vec::new();

                let mut correct_order = true;
                page_list.iter().for_each(|page| {
                    if let Some(index) = processed_pages.iter().position(|p| {
                        if let Some(dependencies) = page_ordering_rules.get(page) {
                            dependencies.contains(p)
                        } else {
                            false
                        }
                    }) {
                        correct_order = false;
                        processed_pages.insert(index, *page);
                    } else {
                        processed_pages.push(*page);
                    }
                });
                if check_for_correct_order {
                    if correct_order {
                        Some(processed_pages)
                    } else {
                        None
                    }
                } else if !correct_order {
                    Some(processed_pages)
                } else {
                    None
                }
            } else {
                let order = line
                    .split('|')
                    .map(|s| s.parse::<u32>().unwrap_or(0))
                    .collect::<Vec<_>>();
                page_ordering_rules
                    .entry(order[0])
                    .or_default()
                    .push(order[1]);
                None // This branch does not produce output for the flat_map
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|pages| {
            let mid_index = pages.len() / 2;
            pages[mid_index]
        })
        .sum::<u32>();
    Some(result)
}
pub fn part_one(input: &str) -> Option<u32> {
    process(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    process(input, false)
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
}
