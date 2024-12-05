advent_of_code::solution!(4);

fn match_xmas(grid: &[Vec<char>], row: &usize, col: &usize, direction: (i32, i32)) -> bool {
    let mut next_row: i32 = *row as i32;
    let mut next_col: i32 = *col as i32;
    let target = ['X', 'M', 'A', 'S'];
    for ch in target.iter() {
        if next_row < 0
            || next_row >= grid.len() as i32
            || next_col < 0
            || next_col >= grid[0].len() as i32
        {
            return false;
        }
        if grid[next_row as usize][next_col as usize] != *ch {
            return false;
        }
        next_row += direction.0;
        next_col += direction.1;
    }
    true
}

fn match_x_mas(
    grid: &[Vec<char>],
    row: &usize,
    col: &usize,
    direction_left_x: ((i32, i32), (i32, i32)),
    direction_right_x: ((i32, i32), (i32, i32)),
) -> bool {
    let target = ['M', 'S'];
    let left_x_top: (i32, i32) = (
        *row as i32 + direction_left_x.0 .0,
        *col as i32 + direction_left_x.0 .1,
    );
    let left_x_bottom: (i32, i32) = (
        *row as i32 + direction_left_x.1 .0,
        *col as i32 + direction_left_x.1 .1,
    );

    let right_x_top: (i32, i32) = (
        *row as i32 + direction_right_x.0 .0,
        *col as i32 + direction_right_x.0 .1,
    );
    let right_x_bottom: (i32, i32) = (
        *row as i32 + direction_right_x.1 .0,
        *col as i32 + direction_right_x.1 .1,
    );

    if left_x_top.0 < 0
        || left_x_top.1 < 0
        || left_x_bottom.0 >= grid[0].len() as i32
        || left_x_bottom.1 >= grid[0].len() as i32
    {
        return false;
    }
    if right_x_top.0 < 0
        || right_x_top.1 >= grid[0].len() as i32
        || right_x_bottom.0 < 0
        || right_x_bottom.1 >= grid[0].len() as i32
    {
        return false;
    }

    fn check_targets(
        grid: &[Vec<char>],
        pos1: (i32, i32),
        pos2: (i32, i32),
        target: &[char],
    ) -> bool {
        let val1 = grid[pos1.0 as usize][pos1.1 as usize];
        let val2 = grid[pos2.0 as usize][pos2.1 as usize];
        (val1 == target[0] && val2 == target[1]) || (val1 == target[1] && val2 == target[0])
    }

    let left_matches = check_targets(grid, left_x_top, left_x_bottom, &target);
    let right_matches = check_targets(grid, right_x_top, right_x_bottom, &target);

    if left_matches && right_matches {
        return true;
    }

    false
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (1, 0),
    ];
    let mut result: u32 = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'X' {
                for direction in directions.iter() {
                    if match_xmas(&grid, &i, &j, *direction) {
                        result += 1;
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result: u32 = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'A'
                && match_x_mas(&grid, &i, &j, ((-1, -1), (1, 1)), ((-1, 1), (1, -1)))
            {
                result += 1;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
