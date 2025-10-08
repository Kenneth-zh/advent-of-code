advent_of_code::solution!(6);

use rayon::prelude::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (mut row, mut col, mut direction) = find_guard(&grid);
    let mut visited = HashSet::new();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    loop {
        visited.insert((row, col));

        let (dr, dc) = get_direction_delta(direction);
        let next_row = row as i32 + dr;
        let next_col = col as i32 + dc;

        // 越界就停止
        if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
            break;
        }

        let next_row = next_row as usize;
        let next_col = next_col as usize;

        // 遇到障碍物就转向
        if grid[next_row][next_col] == '#' {
            direction = turn_right(direction);
        } else {
            row = next_row;
            col = next_col;
        }
    }

    Some(visited.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start_row, start_col, start_dir) = find_guard(&grid);
    let positions: Vec<(usize, usize)> = (0..grid.len())
        .flat_map(|row| (0..grid[0].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == '.')
        .collect();

    let count = positions
        .par_iter() // 使用 par_iter() 并行迭代
        .filter(|&&(row, col)| {
            // 每个线程都需要自己的 grid 副本
            let mut test_grid = grid.clone();
            test_grid[row][col] = '#';
            has_loop(&test_grid, start_row, start_col, start_dir)
        })
        .count();

    Some(count as u64)
}

fn find_guard(grid: &[Vec<char>]) -> (usize, usize, char) {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if matches!(ch, '^' | 'v' | '<' | '>') {
                return (row, col, ch);
            }
        }
    }
    unreachable!()
}

fn get_direction_delta(dir: char) -> (i32, i32) {
    match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!(),
    }
}

fn turn_right(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

fn has_loop(grid: &[Vec<char>], start_row: usize, start_col: usize, start_dir: char) -> bool {
    let mut visited_states = HashSet::new();
    let mut row = start_row;
    let mut col = start_col;
    let mut direction = start_dir;

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    loop {
        // 如果这个状态（位置+方向）已经访问过，说明有循环
        if !visited_states.insert((row, col, direction)) {
            return true;
        }

        let (dr, dc) = get_direction_delta(direction);
        let next_row = row as i32 + dr;
        let next_col = col as i32 + dc;

        // 越界就没有循环
        if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
            return false;
        }

        let next_row = next_row as usize;
        let next_col = next_col as usize;

        // 遇到障碍物就转向
        if grid[next_row][next_col] == '#' {
            direction = turn_right(direction);
        } else {
            row = next_row;
            col = next_col;
        }
    }
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
