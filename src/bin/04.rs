use std::iter::Enumerate;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let target: &str = "XMAS";
    let mut count: u64 = 0;
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for (dr, dc) in directions {
                if search(&grid, dr, dc, row, col, target) {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0u64;
    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            if cross(&grid, row, col) {
                count += 1;
            }
        }
    }
    Some(count)
}

fn search(grid: &Vec<Vec<char>>, dr: i32, dc: i32, row: usize, col: usize, target: &str) -> bool {
    let target: Vec<char> = target.chars().collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, &expected_char) in target.iter().enumerate() {
        let new_row = row as i32 + i as i32 * dr;
        let new_col = col as i32 + i as i32 * dc;

        // 检查边界
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        // 检查字符是否匹配
        if grid[new_row as usize][new_col as usize] != expected_char {
            return false;
        }
    }

    true
}

fn cross(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }

    // 检查两条对角线
    let diag1 = (
        grid[row - 1][col - 1], // 左上
        grid[row][col],         // 中心 A
        grid[row + 1][col + 1], // 右下
    );

    let diag2 = (
        grid[row - 1][col + 1], // 右上
        grid[row][col],         // 中心 A
        grid[row + 1][col - 1], // 左下
    );

    let valid = [('M', 'A', 'S'), ('S', 'A', 'M')];

    valid.contains(&diag1) && valid.contains(&diag2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
