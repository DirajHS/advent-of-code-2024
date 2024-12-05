use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = input
        .lines()
        .flat_map(|line| {
            mul_regex
                .captures_iter(line)
                .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut should_process_mul: bool = true;
    let result = input
        .lines()
        .map(|line| {
            mul_regex
                .captures_iter(line)
                .map(|cap| match &cap[0] {
                    "don't()" => {
                        should_process_mul = false;
                        0
                    }
                    "do()" => {
                        should_process_mul = true;
                        0
                    }
                    _ => {
                        if should_process_mul {
                            cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
                        } else {
                            0
                        }
                    }
                })
                .sum::<u32>()
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
