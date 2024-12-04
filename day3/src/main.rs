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

/// Main body for solving
fn solve(input: &str) -> String {
    let input = input.as_bytes();
    let e = parse_expression(input);
    eval(&e).to_string()
}

fn solve_complex(input: &str) -> String {
    let input = input.as_bytes();
    let e = parse_expression(input);
    eval_control(&e).to_string()
}

#[derive(Eq, PartialEq, Debug)]
enum Expression {
    Mul {
        next_pos: usize,
        p1: i32,
        p2: i32,
    },
    Parens {
        next_pos: usize,
        p1: Option<i32>,
        p2: Option<i32>,
    },
    Do {
        next_pos: usize,
    },
    Dont {
        next_pos: usize,
    },
    Ignore,
}

fn parse_parens(expr: &[u8]) -> Expression {
    if expr.starts_with(b"()") {
        return Expression::Parens {
            next_pos: expr.len(),
            p1: None,
            p2: None,
        };
    }

    let mut iter = expr.iter().enumerate();
    if let Some((_, c)) = iter.next() {
        if c != &b'(' {
            return Expression::Ignore;
        }
    }

    let mut end_p1 = 0;
    let mut start_p2 = 0;
    let mut end_p2 = 0;

    for (pos, c) in iter {
        if c == &b',' && end_p2 == 0 {
            end_p1 = pos;
            start_p2 = pos + 1;
            end_p2 = start_p2 + 1;
        } else if c == &b')' {
            end_p2 = pos;
            break;
        } else if !c.is_ascii_digit() {
            return Expression::Ignore;
        }
    }

    Expression::Parens {
        next_pos: end_p2 + 1,
        p1: Some(String::from_utf8_lossy(&expr[1..end_p1]).parse().unwrap()),
        p2: Some(
            String::from_utf8_lossy(&expr[start_p2..end_p2])
                .parse()
                .unwrap(),
        ),
    }
}

fn parse_mul(expr: &[u8]) -> Expression {
    let ins_bytes = "mul".as_bytes();

    if !expr.starts_with(ins_bytes) {
        return Expression::Ignore;
    }

    let ins_len = ins_bytes.len();
    match parse_parens(&expr[ins_len..]) {
        // Match only when both params are present
        Expression::Parens {
            next_pos,
            p1: Some(p1),
            p2: Some(p2),
        } => Expression::Mul {
            next_pos: next_pos + ins_len,
            p1,
            p2,
        },
        _ => Expression::Ignore,
    }
}

fn peek_dont(expr: &[u8]) -> Expression {
    let ins_bytes = "don't()".as_bytes();

    match expr.starts_with(ins_bytes) {
        true => Expression::Dont {
            next_pos: ins_bytes.len(),
        },
        _ => Expression::Ignore,
    }
}

fn parse_do(expr: &[u8]) -> Expression {
    let ins_bytes = "do()".as_bytes();

    match expr.starts_with(ins_bytes) {
        true => Expression::Do {
            next_pos: ins_bytes.len(),
        },
        _ => Expression::Ignore,
    }
}

fn eval(exprs: &[Expression]) -> i32 {
    let mut result = 0;

    for e in exprs {
        match e {
            Expression::Mul {
                next_pos: _,
                p1,
                p2,
            } => result += p1 * p2,
            _ => continue,
        }
    }

    result
}

fn eval_control(exprs: &[Expression]) -> i32 {
    let mut result = 0;
    let mut do_mul = true;

    for e in exprs {
        match e {
            Expression::Mul {
                next_pos: _,
                p1,
                p2,
            } => {
                if do_mul {
                    result += p1 * p2;
                }
            }
            Expression::Do { .. } => {
                do_mul = true;
            }
            Expression::Dont { .. } => {
                do_mul = false;
            }
            _ => continue,
        }
    }

    result
}

fn parse_expression(input: &[u8]) -> Vec<Expression> {
    let mut it = input.iter().enumerate();
    let mut tree = vec![];

    while let Some((i, c)) = it.next() {
        let expr = parse_subexpr_from_tok(input, i, c);

        match expr {
            Expression::Mul {
                next_pos,
                p1: _,
                p2: _,
            } => {
                it.nth(next_pos - 2);
                tree.push(expr);
            }
            Expression::Do { next_pos } => {
                it.nth(next_pos - 2);
                tree.push(expr);
            }
            Expression::Dont { next_pos } => {
                it.nth(next_pos - 2);
                tree.push(expr);
            }
            _ => {}
        };
    }

    tree
}

fn parse_subexpr_from_tok(input: &[u8], i: usize, c: &u8) -> Expression {
    match c {
        b'm' => parse_mul(&input[i..]),
        b'd' => match peek_dont(&input[i..]) {
            Expression::Ignore => parse_do(&input[i..]),
            dont_expr => dont_expr,
        },
        _ => Expression::Ignore,
    }
}

#[cfg(test)]
mod validation {
    use crate::{
        parse_do, parse_mul, parse_parens, peek_dont, solve, solve_complex, Expression, DAY,
        WORKSPACE,
    };

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
        let input = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/sample2.txt"))
            .expect("Can load input simple");

        let expected = std::fs::read_to_string(format!("{WORKSPACE}/../inputs/{DAY}/answer_2.txt"))
            .expect("Can load answers simple");

        assert_eq!(solve_complex(&input), expected);
    }

    #[test]
    fn test_parens() {
        assert_eq!(
            parse_parens(&"(123,456)".as_bytes()),
            Expression::Parens {
                next_pos: 9,
                p1: Some(123),
                p2: Some(456)
            }
        );
        assert_eq!(
            parse_parens(&"()".as_bytes()),
            Expression::Parens {
                next_pos: 2,
                p1: None,
                p2: None
            }
        );

        let input_invalid = [
            "( 123 , 456 )",
            "mul(4*",
            "mul(6,9!",
            "?(12,34)",
            "(12,,34)",
            "(12 34)",
            "((12,,34))",
        ];
        assert!(input_invalid
            .iter()
            .map(|input| parse_parens(&input.as_bytes()))
            .all(|value| value == Expression::Ignore));
    }

    #[test]
    fn test_peek_mul() {
        let input_valid = "mul(123,456)";
        assert_eq!(
            parse_mul(&input_valid.as_bytes()),
            Expression::Mul {
                next_pos: 12,
                p1: 123,
                p2: 456
            }
        );

        let input_invalid = [
            "mul( 123 , 456 )",
            "mul(4*",
            "mul(6,9!",
            "mul?(12,34)",
            "mul(12,,34)",
            "mul(12 34)",
            "mul((12,,34))",
        ];
        assert!(input_invalid
            .iter()
            .map(|input| parse_parens(&input.as_bytes()))
            .all(|value| value == Expression::Ignore));
    }

    #[test]
    fn test_peek_dont() {
        let input_valid = "don't()";
        assert_eq!(
            peek_dont(&input_valid.as_bytes()),
            Expression::Dont { next_pos: 7 }
        );

        let input_invalid = ["don't( )", "don't(())"];
        assert!(input_invalid
            .iter()
            .map(|input| parse_parens(&input.as_bytes()))
            .all(|value| value == Expression::Ignore));
    }

    #[test]
    fn test_peek_do() {
        let input_valid = "do()";
        assert_eq!(
            parse_do(&input_valid.as_bytes()),
            Expression::Do { next_pos: 4 }
        );

        let input_invalid = ["do( )", "do(())"];
        assert!(input_invalid
            .iter()
            .map(|input| parse_parens(&input.as_bytes()))
            .all(|value| value == Expression::Ignore));
    }
}
