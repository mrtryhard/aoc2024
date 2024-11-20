/// The following is the crate name in Cargo.toml
pub const DAY: &'static str = env!("CARGO_PKG_NAME");
pub const WORKSPACE: &'static str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}.txt"))
        .expect("Can load input simple");
    
    println!("Answer: {}", solve(&input));
}

/// Main body for solving
fn solve(input: &str) -> String {
    "".to_owned()
}

#[cfg(test)]
mod validation {
    use crate::{solve, DAY, WORKSPACE};

    #[test]
    fn case_simple() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_simple.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_simple_answer.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve(&input), expected);
    }

    #[test]
    fn case_complex() {
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_complex.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}_complex_answer.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve(&input), expected);
    }
}