use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: u64 = re
        .captures_iter(input)
        .map(|caps| {
            let x: u64 = caps[1].parse().unwrap();
            let y: u64 = caps[2].parse().unwrap();
            x * y
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0u64;

    for caps in re.captures_iter(input) {
        let instruction = &caps[0];

        match instruction {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ if instruction.starts_with("mul(") && enabled => {
                let x: u64 = caps[1].parse().unwrap();
                let y: u64 = caps[2].parse().unwrap();
                sum += x * y;
            }
            _ => {}
        }
    }

    Some(sum)
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
