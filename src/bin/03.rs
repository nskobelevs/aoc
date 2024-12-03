use regex::Regex;

advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Dont,
    Do,
    Mul(u32, u32),
}

fn parse_instructions(input: &str) -> Vec<Op> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

    re.captures_iter(input)
        .map(|captures| {
            let op = captures.get(0).map(|c| c.as_str()).unwrap();

            if op == "don't()" {
                Op::Dont
            } else if op == "do()" {
                Op::Do
            } else if op.starts_with("mul(") {
                let a: u32 = captures
                    .get(1)
                    .map(|c| c.as_str())
                    .and_then(|s| s.parse().ok())
                    .unwrap();

                let b: u32 = captures
                    .get(2)
                    .map(|c| c.as_str())
                    .and_then(|s| s.parse().ok())
                    .unwrap();

                Op::Mul(a, b)
            } else {
                unreachable!()
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_instructions(input)
            .iter()
            .map(|op| match op {
                Op::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse_instructions(input)
        .iter()
        .fold((0, true), |(acc, active), op| match (*op, active) {
            (Op::Dont, true) => (acc, false),
            (Op::Do, false) => (acc, true),
            (Op::Mul(a, b), true) => (acc + a * b, true),
            _ => (acc, active),
        });

    Some(result.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
