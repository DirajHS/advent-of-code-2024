advent_of_code::solution!(2);

fn is_safe_report(report: &str) -> bool {
    let levels: Vec<u32> = report
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    is_safe(levels)
}
pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input.lines().filter(|line| is_safe_report(line)).count();

    Some(safe_reports as u32)
}

fn is_safe(levels: Vec<u32>) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let is_increasing = levels
        .windows(2)
        .all(|pair| pair[1] > pair[0] && pair[1] - pair[0] <= 3);
    let is_decreasing = levels
        .windows(2)
        .all(|pair| pair[1] < pair[0] && pair[0] - pair[1] <= 3);

    is_increasing || is_decreasing
}
fn is_safe_or_dampened(report: &str) -> bool {
    if is_safe_report(report) {
        return true;
    }
    let levels: Vec<u32> = report
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    for i in 0..levels.len() {
        let mut modified = levels.clone();
        modified.remove(i);
        if is_safe(modified) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .filter(|line| is_safe_or_dampened(line))
        .count();

    Some(safe_reports as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
