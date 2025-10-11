advent_of_code::solution!(8);
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != '.' {
                nodes
                    .entry(grid[i][j])
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, positions) in nodes {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                // 计算向量差
                let dr = r2 - r1;
                let dc = c2 - c1;

                let antinode1 = (r1 - dr, c1 - dc);
                let antinode2 = (r2 + dr, c2 + dc);

                if is_in_bounds(antinode1, rows as i32, cols as i32) {
                    anti_nodes.insert(antinode1);
                }
                if is_in_bounds(antinode2, rows as i32, cols as i32) {
                    anti_nodes.insert(antinode2);
                }
            }
        }
    }

    Some(anti_nodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != '.' {
                nodes
                    .entry(grid[i][j])
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, positions) in &nodes {
        if positions.len() >= 2 {
            for &pos in positions {
                anti_nodes.insert(pos);
            }
        }

        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                // 计算向量差
                let dr = r2 - r1;
                let dc = c2 - c1;

                let mut antinode1 = (r1 - dr, c1 - dc);
                let mut antinode2 = (r2 + dr, c2 + dc);

                while is_in_bounds(antinode1, rows as i32, cols as i32) {
                    anti_nodes.insert(antinode1);
                    let (a, b) = antinode1;
                    antinode1 = (a - dr, b - dc);
                }
                while is_in_bounds(antinode2, rows as i32, cols as i32) {
                    anti_nodes.insert(antinode2);
                    let (a, b) = antinode2;
                    antinode2 = (a + dr, b + dc);
                }
            }
        }
    }

    Some(anti_nodes.len() as u64)
}

fn is_in_bounds((row, col): (i32, i32), rows: i32, cols: i32) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
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
