use itertools::Itertools;

/// The following is the crate name in Cargo.toml
pub const DAY: &str = env!("CARGO_PKG_NAME");
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/full.txt"))
        .expect("Can load input simple")
        .trim()
        .to_string();

    println!("Answer simple: {}", solve(&input));
    println!("Answer complex: {}", solve_complex(&input));
}

fn value_in_range(value: u32) -> bool {
    value > 0 && value < 4
}

fn is_report_safe(levels: &[i32]) -> bool {
    levels.iter()
        .tuple_windows()
        .all(|(left, right)| value_in_range(left.abs_diff(*right)))
        &&
        (levels.is_sorted() || levels.iter().rev().is_sorted())
}

fn is_report_safe_violations(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut sub = Vec::from(&levels[0..i]);
        sub.extend_from_slice(&levels[i+1..]);

        if is_report_safe(&sub) {
            return true;
        }
    }

    false
}

/// Main body for solving
fn solve(input: &str) -> String {
    input
        .split("\n")
        .map(|report_str| {
            report_str
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|a| is_report_safe(a))
        .count()
        .to_string()
}

fn solve_complex(input: &str) -> String {
    input
        .split("\n")
        .map(|report_str| {
            report_str
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|a| is_report_safe_violations(a))
        .count()
        .to_string()
}

#[cfg(test)]
mod validation {
    use crate::{solve, solve_complex, DAY, WORKSPACE};

    #[test]
    fn case_simple() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/sample.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/answer_1.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve(&input), expected);
    }

    #[test]
    fn case_complex() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/sample.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/answer_2.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve_complex(&input), expected);
    }

    #[test]
    fn case_complex_extra() {
        // OK -> [47] 47 46 43 || 47 [47] 46 43
        // NOK
        // NOK
        // NOK
        let input = r#"47 47 46 43
95 95 95 94 96
48 48 48 45 44 44
18 18 18 16 14 10
18 15 16 18 18
78 78"#;
        let expected = 2.to_string();
        assert_eq!(solve_complex(&input), expected);
    }
}
