advent_of_code::solution!(10);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let mut sum = 0u64;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                sum += find_unique_peaks(i, j, &grid);
            }
        }
    }
    Some(sum)
}

fn find_unique_peaks(row: usize, col: usize, grid: &Vec<Vec<u64>>) -> u64 {
    let mut visited_peaks = HashSet::new();
    dfs_unique_peaks(row, col, grid, &mut visited_peaks);
    visited_peaks.len() as u64
}

fn dfs_unique_peaks(
    row: usize,
    col: usize,
    grid: &Vec<Vec<u64>>,
    peaks: &mut HashSet<(usize, usize)>,
) {
    let current_height = grid[row][col];

    // 基础情况：到达山峰（高度 9）
    if current_height == 9 {
        peaks.insert((row, col)); // 记录这个山峰的位置
        return;
    }

    // 递归情况：探索四个方向
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // 边界检查
        if new_row >= 0
            && new_row < grid.len() as i32
            && new_col >= 0
            && new_col < grid[0].len() as i32
        {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // 高度必须恰好增加 1
            if grid[new_row][new_col] == current_height + 1 {
                dfs_unique_peaks(new_row, new_col, grid, peaks);
            }
        }
    }
}

fn find_route(row: usize, col: usize, grid: &Vec<Vec<u64>>) -> u64 {
    let mut count = 0u64;
    dfs(row, col, grid, &mut count);
    count
}

fn dfs(row: usize, col: usize, grid: &Vec<Vec<u64>>, count: &mut u64) {
    let current_height = grid[row][col];

    // 基础情况：到达山峰（高度 9）
    if current_height == 9 {
        *count += 1;
        return;
    }

    // 递归情况：探索四个方向
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // 右、左、下、上

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // 边界检查
        if new_row >= 0
            && new_row < grid.len() as i32
            && new_col >= 0
            && new_col < grid[0].len() as i32
        {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // 高度必须恰好增加 1
            if grid[new_row][new_col] == current_height + 1 {
                dfs(new_row, new_col, grid, count);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
