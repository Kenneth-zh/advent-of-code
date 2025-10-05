advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let count = input.lines().filter(|line| is_safe(line)).count() as u64;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .filter(|line| is_safe_with_dampener(line))
        .count() as u64;
    Some(count)
}

fn is_safe(line: &str) -> bool {
    let numbers: Vec<i32> = line.split(' ').filter_map(|num| num.parse().ok()).collect();
    is_safe_numbers(&numbers)
}

fn is_safe_with_dampener(line: &str) -> bool {
    let numbers: Vec<i32> = line.split(' ').filter_map(|num| num.parse().ok()).collect();

    if is_safe_numbers(&numbers) {
        return true;
    };

    for index in 0..numbers.len() {
        let mut temp = numbers.clone();
        temp.remove(index);
        if is_safe_numbers(&temp) {
            return true;
        }
    }

    false
}

fn is_safe_numbers(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let in_range = diffs.iter().all(|diff| diff.abs() <= 3 && diff.abs() >= 1);

    let is_monotonic = diffs.iter().all(|diff| diff > &0) || diffs.iter().all(|diff| diff < &0);

    in_range && is_monotonic
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
