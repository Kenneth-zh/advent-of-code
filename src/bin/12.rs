use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|str| str.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut plant: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            plant.entry(grid[i][j]).or_insert(Vec::new()).push((i, j));
        }
    }

    let res = plant
        .iter()
        .map(|(_, list)| length(list) * list.len() as u64)
        .sum();

    Some(res)
}

fn length(list: &Vec<(usize, usize)>) -> u64 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut result = 0u64;
    for (row, col) in list {
        let mut temp = 4u64;
        for (dr, dc) in directions {
            let new_row = *row as i32 + dr;
            let new_col = *col as i32 + dc;

            if new_row >= 0 && new_col >= 0 {
                if list.contains(&(new_row as usize, new_col as usize)) {
                    temp -= 1;
                }
            }
        }
        result += temp;
    }
    result
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
