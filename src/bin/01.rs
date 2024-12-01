use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        if nums.len() == 2 {
            left.push(nums[0]);
            right.push(nums[1]);
        }
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = parse_lists(input).0;
    let mut right = parse_lists(input).1;

    left.sort();
    right.sort();

    let total_distance = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let left = parse_lists(input).0;
    let right = parse_lists(input).1;

    let mut right_frequency = HashMap::new();
    for &num in &right {
        *right_frequency.entry(num).or_insert(0) += 1;
    }
    let similarity_score: u32 = left
        .iter()
        .map(|a| a * right_frequency.get(a).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
