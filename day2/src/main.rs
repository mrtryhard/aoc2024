use std::iter::zip;

/// The following is the crate name in Cargo.toml
pub const DAY: &str = env!("CARGO_PKG_NAME");
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/full.txt"))
        .expect("Can load input simple");

    println!("Answer simple: {}", solve(&input));
    println!("Answer complex: {}", solve_complex(&input));
}

/// Main body for solving
fn solve(input: &str) -> String {
    "".to_string()
}

fn solve_complex(input: &str) -> String {
    "".to_string()
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
}
