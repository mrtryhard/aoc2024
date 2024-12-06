use std::collections::HashSet;

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
    let (rules_block, requests_block) = input.trim().split_once("\n\n").unwrap();
    let rules = parse_rules(rules_block);
    let requests = parse_requests(requests_block);

    requests
        .iter()
        .filter(|request| request_valid(&rules, request))
        .map(|request| {
            let middle = request.len() / 2;
            request[middle]
        })
        .sum::<usize>()
        .to_string()
}

fn solve_complex(input: &str) -> String {
    let (rules_block, requests_block) = input.trim().split_once("\n\n").unwrap();
    let rules = parse_rules(rules_block);
    let requests = parse_requests(requests_block);

    requests
        .iter()
        .filter(|request| !request_valid(&rules, request))
        .map(|request| reorder(&rules, request))
        .map(|request| {
            let middle = request.len() / 2;
            request[middle]
        })
        .sum::<usize>()
        .to_string()
}

fn parse_rules(rules_block: &str) -> Vec<Vec<usize>> {
    let mut rules_vec: Vec<Vec<usize>> = vec![];
    rules_vec.resize(10000, Vec::<usize>::new());

    rules_block.trim().split("\n").for_each(|rule| {
        let (come_first, come_after) = rule.split_once("|").unwrap();
        rules_vec[come_first.parse::<usize>().unwrap()].push(come_after.parse::<usize>().unwrap());
    });
    rules_vec
}

fn parse_requests(requests_block: &str) -> Vec<Vec<usize>> {
    let requests: Vec<Vec<usize>> = requests_block
        .trim()
        .split("\n")
        .map(|request| {
            request
                .split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<_>();
    requests
}

fn request_valid(rules: &[Vec<usize>], request: &[usize]) -> bool {
    let mut processed = HashSet::new();

    request
        .iter()
        .map(|page| {
            let page_rule = rules.get(*page).unwrap();
            let invalid = &page_rule.iter().any(|rule| processed.contains(rule));
            processed.insert(page);
            *invalid
        })
        .all(|b| !b)
}

fn reorder(rules: &[Vec<usize>], request: &[usize]) -> Vec<usize> {
    let mut new_request = Vec::with_capacity(request.len());

    request.iter().for_each(|page| {
        let page_rule = rules.get(*page).unwrap();
        let invalid = &new_request
            .iter()
            .position(|rule| page_rule.iter().any(|x| x == rule));

        if let Some(pos) = invalid {
            new_request.insert(*pos, *page);
        } else {
            new_request.push(*page);
        }
    });

    new_request
}

#[cfg(test)]
mod validation {
    use crate::{reorder, request_valid, solve, solve_complex, DAY, WORKSPACE};

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
    fn tst_validate_request() {
        let rules = vec![
            vec![3], // 0 is before 3
            vec![0], // 1 is before 0
            vec![],  // no rules for 2
            vec![2], // 3 is before 2
        ];

        assert!(request_valid(&rules, &vec![1, 0, 3, 2]));
        assert!(request_valid(&rules, &vec![1]));
        assert!(request_valid(&rules, &vec![3, 2]));
        assert!(request_valid(&rules, &vec![2]));
        assert!(!request_valid(&rules, &vec![2, 3]));
    }

    #[test]
    fn tst_reorder() {
        let rules = vec![
            vec![3], // 0 is before 3
            vec![0], // 1 is before 0
            vec![],  // no rules for 2
            vec![2], // 3 is before 2
        ];

        assert_eq!(reorder(&rules, &vec![2, 3]), vec![3, 2]);
        assert_eq!(reorder(&rules, &vec![0, 1, 2, 3]), vec![1, 0, 3, 2]);
    }
}
