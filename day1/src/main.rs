use std::iter::zip;

/// The following is the crate name in Cargo.toml
pub const DAY: &str = env!("CARGO_PKG_NAME");
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}.txt"))
        .expect("Can load input simple");

    println!("Answer simple: {}", solve(&input));
    println!("Answer complex: {}", solve_complex(&input));
}

/// Main body for solving
fn solve(input: &str) -> String {
    let (mut left_list, mut right_list) = parse_lists(input);

    left_list.sort();
    right_list.sort();

    let total: i32 = zip(left_list, right_list)
        .map(|(x, y)| x.max(y) - x.min(y))
        .sum();

    total.to_string()
}

fn solve_complex(input: &str) -> String {
    let (left_list, right_list) = parse_lists(input);

    let similarities = left_list
        .iter()
        .map(|x| right_list.iter().filter(|y| *y == x).count() as i32)
        .collect::<Vec<i32>>();

    let total: i32 = zip(left_list, similarities).map(|(x, y)| x * y).sum();

    total.to_string()
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let left_list: Vec<i32> = input
        .split("\n")
        .map(|str| {
            str.split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect();

    let right_list: Vec<i32> = input
        .split("\n")
        .map(|str| {
            str.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect();
    (left_list, right_list)
}

#[cfg(test)]
mod validation {
    use crate::{solve, solve_complex, DAY, WORKSPACE};

    #[test]
    fn case_simple() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_sample.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_answer_1.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve(&input), expected);
    }

    #[test]
    fn case_complex() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_sample.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_answer_2.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve_complex(&input), expected);
    }
}
