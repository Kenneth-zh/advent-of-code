advent_of_code::solution!(10);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let trailheads = find_trailheads(&grid);

    let total_score: u64 = trailheads
        .iter()
        .map(|&pos| calculate_score(&grid, pos))
        .sum();

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let trailheads = find_trailheads(&grid);

    let total_rating: u64 = trailheads
        .iter()
        .map(|&pos| calculate_rating(&grid, pos))
        .sum();

    Some(total_rating)
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect()
        })
        .collect()
}

fn find_trailheads(grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, &height) in line.iter().enumerate() {
            if height == 0 {
                trailheads.push((row, col));
            }
        }
    }

    trailheads
}

// Part 1: 计算能到达多少个不同的终点
fn calculate_score(grid: &[Vec<u32>], start: (usize, usize)) -> u64 {
    let mut reachable_nines = HashSet::new();
    dfs_unique_endpoints(grid, start.0, start.1, 0, &mut reachable_nines);
    reachable_nines.len() as u64
}

// Part 2: 计算有多少条不同的路径
fn calculate_rating(grid: &[Vec<u32>], start: (usize, usize)) -> u64 {
    dfs_count_paths(grid, start.0, start.1, 0)
}

fn dfs_unique_endpoints(
    grid: &[Vec<u32>],
    row: usize,
    col: usize,
    expected_height: u32,
    reachable_nines: &mut HashSet<(usize, usize)>,
) {
    // 边界检查
    if row >= grid.len() || col >= grid[0].len() {
        return;
    }

    // 高度检查
    if grid[row][col] != expected_height {
        return;
    }

    // 到达终点
    if expected_height == 9 {
        reachable_nines.insert((row, col));
        return;
    }

    // 探索四个方向
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_col >= 0 {
            dfs_unique_endpoints(
                grid,
                new_row as usize,
                new_col as usize,
                expected_height + 1,
                reachable_nines,
            );
        }
    }
}

fn dfs_count_paths(grid: &[Vec<u32>], row: usize, col: usize, expected_height: u32) -> u64 {
    // 边界检查
    if row >= grid.len() || col >= grid[0].len() {
        return 0;
    }

    // 高度检查
    if grid[row][col] != expected_height {
        return 0;
    }

    // 到达终点
    if expected_height == 9 {
        return 1;
    }

    // 计算从四个方向来的路径总数
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut total_paths = 0;

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_col >= 0 {
            total_paths += dfs_count_paths(
                grid,
                new_row as usize,
                new_col as usize,
                expected_height + 1,
            );
        }
    }

    total_paths
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
