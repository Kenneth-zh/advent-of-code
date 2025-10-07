advent_of_code::solution!(5);

use core::num;
use std::{collections::HashMap, io::Lines};

pub fn part_one(input: &str) -> Option<u64> {
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut seq: Vec<Vec<u64>> = Vec::new();
    let mut flag = true;
    for line in input.lines() {
        if line.trim().is_empty() {
            flag = false;
            continue;
        }

        if flag {
            let numbers: Vec<u64> = line
                .split('|')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();

            rules
                .entry(numbers[0])
                .or_insert(Vec::new())
                .push(numbers[1]);
        } else {
            seq.push(
                line.split(',')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect(),
            );
        }
    }

    let mut sum = 0;
    for update in seq {
        if is_valid(&update, &rules) {
            let middle = update[update.len() / 2];
            sum += middle;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut seq: Vec<Vec<u64>> = Vec::new();
    let mut flag = true;
    for line in input.lines() {
        if line.trim().is_empty() {
            flag = false;
            continue;
        }

        if flag {
            let numbers: Vec<u64> = line
                .split('|')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();

            rules
                .entry(numbers[0])
                .or_insert(Vec::new())
                .push(numbers[1]);
        } else {
            seq.push(
                line.split(',')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect(),
            );
        }
    }

    let mut sum = 0;
    for update in seq {
        if !is_valid(&update, &rules) {
            let temp = fix(update, &rules);
            let middle = temp[temp.len() / 2];
            sum += middle;
        }
    }

    Some(sum)
}

fn is_valid(update: &Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let current = update[i];
            let later = update[j];

            if let Some(must_be_after_later) = rules.get(&later) {
                if must_be_after_later.contains(&current) {
                    return false;
                }
            }
        }
    }
    true
}

fn is_valid_fun(update: &Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> bool {
    !update.windows(2).any(|pair| {
        let (current, next) = (pair[0], pair[1]);
        rules
            .get(&next)
            .map_or(false, |must_be_after| must_be_after.contains(&current))
    })
}

fn fix(mut update: Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let len = update.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            let current = update[j];
            let next = update[j + 1];

            if let Some(rule) = rules.get(&next) {
                if rule.contains(&current) {
                    update.swap(j, j + 1);
                }
            }
        }
    }

    update
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
