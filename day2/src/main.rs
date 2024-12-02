#![feature(array_windows)]

use std::str::FromStr;

type Level = i64;
type Report = Vec<Level>;

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(parse_report).collect()
}

fn parse_report(line: &str) -> Report {
    line.split_whitespace()
        .map(|chars| Level::from_str(chars).unwrap())
        .collect()
}

fn is_report_safe_with_dampener(report: &Report) -> bool {
    if is_report_safe(report) {
        return true;
    }

    // Dampen by removing one item.
    (0..report.len()).any(|i| is_report_safe(&dampen(report, i)))
}

// Dampen item i of the report.
fn dampen(report: &Report, i: usize) -> Report {
    let mut dampened = report.clone();
    dampened.remove(i);
    dampened
}

fn is_report_safe(report: &Report) -> bool {
    // All increasing OR all decreasing (no consecutive numbers can be the same)
    // Gap between 1 and 3 (inclusive)
    let (increasing, decreasing, size_ok) = report
        .array_windows()
        .map(|[level0, level1]| level1 - level0) // Find the gap between two levels
        .fold(
            (true, true, true),
            |(increasing, decreasing, size_ok), gap| {
                (
                    increasing && gap > 0,
                    decreasing && gap < 0,
                    size_ok && (1..=3).contains(&gap.abs()),
                )
            },
        );
    (increasing || decreasing) && size_ok
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reports = parse_input(&input);
    let q1 = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    let q2 = reports
        .iter()
        .filter(|report| is_report_safe_with_dampener(report))
        .count();
    println!("Q1: {q1}\nQ2: {q2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let reports = parse_input(TEST_INPUT);
        let q1 = reports
            .iter()
            .filter(|report| is_report_safe(report))
            .count();
        assert_eq!(q1, 2);
    }

    #[test]
    fn test_q2() {
        let reports = parse_input(TEST_INPUT);
        let q1 = reports
            .iter()
            .filter(|report| is_report_safe_with_dampener(report))
            .count();
        assert_eq!(q1, 4);
    }

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
}
