advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // let mut chart: Vec<Vec<u64>> = Vec::new();
    // for line in input.lines() {
    //     chart.push(
    //         line.split(|icon| icon == ' ' || icon == ':')
    //             .filter_map(|char| char.parse::<u64>().ok())
    //             .collect(),
    //     );
    // }

    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u64> = line
                .split(|c| c == ' ' || c == ':')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            let target = numbers[0];
            let operands = numbers[1..].to_vec();
            (target, operands)
        })
        .collect();

    let sum: u64 = equations
        .iter()
        .filter(|(target, operands)| valid(*target, operands))
        .map(|(target, _)| target)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u64> = line
                .split(|c| c == ' ' || c == ':')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            let target = numbers[0];
            let operands = numbers[1..].to_vec();
            (target, operands)
        })
        .collect();

    let sum: u64 = equations
        .iter()
        .filter(|(target, operands)| valid2(*target, operands))
        .map(|(target, _)| target)
        .sum();

    Some(sum)
}

fn valid(target: u64, operands: &[u64]) -> bool {
    if operands.len() < 2 {
        return false;
    }

    try_operations(operands[0], &operands[1..], target)
}

fn valid2(target: u64, operands: &[u64]) -> bool {
    if operands.len() < 2 {
        return false;
    }

    try_operations2(operands[0], &operands[1..], target)
}

fn try_operations(current: u64, remaining: &[u64], target: u64) -> bool {
    if remaining.is_empty() {
        return target == current;
    }

    let next = remaining[0];
    let rest = &remaining[1..];

    if current > target {
        return false;
    }

    try_operations(next * current, rest, target) || try_operations(current + next, rest, target)
}

fn try_operations2(current: u64, remaining: &[u64], target: u64) -> bool {
    if remaining.is_empty() {
        return target == current;
    }

    let next = remaining[0];
    let rest = &remaining[1..];

    if current > target {
        return false;
    }

    try_operations2(next * current, rest, target)
        || try_operations2(current + next, rest, target)
        || try_operations2(cat(current, next), rest, target)
}

fn cat(a: u64, b: u64) -> u64 {
    let digits = count_digits(b);
    a * 10_u64.pow(digits) + b
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
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
