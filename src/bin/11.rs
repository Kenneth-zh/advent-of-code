advent_of_code::solution!(11);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(simulate_efficient(&stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(simulate_efficient(&stones, 75))
}

fn simulate_efficient(stones: &[u64], blinks: usize) -> u64 {
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    fn count_stones(
        n: u64,
        cache: &mut HashMap<(u64, usize), u64>,
        remaining_blinks: usize,
    ) -> u64 {
        if remaining_blinks == 0 {
            return 1;
        }

        if let Some(&cached) = cache.get(&(n, remaining_blinks)) {
            return cached;
        }

        let result = if n == 0 {
            count_stones(1, cache, remaining_blinks - 1)
        } else if has_even_digits(n) {
            let (a, b) = split_number(n);
            count_stones(a, cache, remaining_blinks - 1)
                + count_stones(b, cache, remaining_blinks - 1)
        } else {
            count_stones(n * 2024, cache, remaining_blinks - 1)
        };

        cache.insert((n, remaining_blinks), result);
        result
    }

    stones
        .iter()
        .map(|&stone| count_stones(stone, &mut cache, blinks))
        .sum()
}

fn has_even_digits(n: u64) -> bool {
    count_digits(n) % 2 == 0
}

fn count_digits(n: u64) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as usize + 1
    }
}

fn split_number(n: u64) -> (u64, u64) {
    let digits = count_digits(n);
    let divisor = 10_u64.pow((digits / 2) as u32);
    (n / divisor, n % divisor)
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
