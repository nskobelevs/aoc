advent_of_code::solution!(2);

fn parse_reports(
    input: &str,
) -> impl Iterator<Item = impl Iterator<Item = u32> + use<'_>> + use<'_> {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|part| part.parse().unwrap())
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LevelsTrend {
    Increasing,
    Decreasing,
}

fn part_one_raw<I: Iterator<Item = u32>>(mut report: I) -> bool {
    report
        .try_fold((None, None), |(trend, prev): (_, Option<u32>), current| {
            let current_trend = prev.map(|prev| {
                if current > prev {
                    LevelsTrend::Increasing
                } else {
                    LevelsTrend::Decreasing
                }
            });

            match (prev, trend) {
                // First iteration - set prev
                (None, _) => Some((None, Some(current))),
                // Out of bounds difference - short circuit unsafe
                (Some(prev), _) if !(1..=3).contains(&prev.abs_diff(current)) => None,
                // Second iteration, set prev & trend
                // current_trend != None since not first iteration
                (_, None) => Some((current_trend, Some(current))),
                // Trend matches - keep trend, set prev
                // current_trend != None and trend != None since not first iteration
                (_, trend) if current_trend == trend => Some((current_trend, Some(current))),
                _ => None,
            }
        })
        .is_some()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    Some(reports.map(|report| part_one_raw(report) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    Some(
        reports
            .map(|report| {
                let report = report.collect::<Vec<_>>();

                for missing_index in 0..report.len() {
                    let subsequence = report[0..missing_index]
                        .iter()
                        .chain(report[missing_index + 1..report.len()].iter())
                        .copied()
                        .collect::<Vec<_>>();

                    if part_one_raw(subsequence.into_iter()) {
                        return 1;
                    }
                }

                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
