advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines(){
        let numbers: Vec<u64> = line
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    left_list.sort();
    right_list.sort();

    let distance = left_list.iter().zip(right_list.iter()).map(|(x,y)| if x>y {x-y}else{y-x}).sum();
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left_list = Vec::new();
    let mut right_list = HashMap::new();

    for line in input.lines(){
        let numbers: Vec<u64> = line
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        left_list.push(numbers[0]);
        *right_list.entry(numbers[1]).or_insert(0) += 1;
    }

    let score = left_list.iter().map(|x| x*right_list.get(x).unwrap_or(&0)).sum();
    Some(score)
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
