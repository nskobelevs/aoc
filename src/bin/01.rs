use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_ids(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line
                .split_ascii_whitespace()
                .map(|part| part.parse().unwrap());

            let left: u32 = parts.next().unwrap();
            let right: u32 = parts.next().unwrap();

            (left, right)
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_ids(input);

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_ids(input);

    let right_freq = right.into_iter().counts();

    Some(
        left.into_iter()
            .map(|item| item * *right_freq.get(&item).unwrap_or(&0) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
