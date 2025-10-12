use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price = 0u64;

    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                let (area, perimeter) = calculate_region(&grid, i, j, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    Some(total_price)
}

fn calculate_region(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (u64, u64) {
    let plant_type = grid[start_row][start_col];
    let mut area = 0u64;
    let mut perimeter = 0u64;
    let mut stack = vec![(start_row, start_col)];

    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        area += 1;

        // 检查四个方向
        let directions = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0
                && new_row < grid.len() as i32
                && new_col >= 0
                && new_col < grid[0].len() as i32
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == plant_type {
                    if !visited.contains(&(new_row, new_col)) {
                        stack.push((new_row, new_col));
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
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
