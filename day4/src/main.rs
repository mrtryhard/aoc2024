use std::fmt::{Debug, Formatter, Write};
use std::str::FromStr;

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

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];

/// Main body for solving
fn solve(input: &str) -> String {
    let grid = Grid::from_str(input).unwrap();

    grid.into_iter()
        .filter(|cell| cell.value == XMAS[0])
        .map(|cell| {
            grid.count((cell.row, cell.col), &XMAS, Searcher::Right)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::Left)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::Up)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::Down)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::DownRight)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::DownLeft)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::UpRight)
                + grid.count((cell.row, cell.col), &XMAS, Searcher::UpLeft)
        })
        .sum::<usize>()
        .to_string()
}

fn solve_complex(input: &str) -> String {
    let grid = Grid::from_str(input).unwrap();

    grid.into_iter()
        .filter(|cell| cell.value == MAS[1])
        .filter(|cell| cell.row > 0 && cell.col > 0)
        .map(|cell| {
            grid.count((cell.row - 1, cell.col - 1), &MAS, Searcher::DownRight)
                + grid.count((cell.row + 1, cell.col + 1), &MAS, Searcher::UpLeft)
                + grid.count((cell.row + 1, cell.col - 1), &MAS, Searcher::UpRight)
                + grid.count((cell.row - 1, cell.col + 1), &MAS, Searcher::DownLeft)
        })
        .filter(|value| value == &2)
        .count()
        .to_string()
}

struct Cell {
    row: usize,
    col: usize,
    value: char,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})[{}]", self.row, self.col, self.value))
    }
}

struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for cell in self {
            f.write_fmt(format_args!("{},", cell.value))?;
            if cell.col == self.cols - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn at(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.grid[row][col].value)
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(&self.grid[row][col])
        }
    }

    pub fn count(&self, pos: (usize, usize), word: &[char], iter: Searcher) -> usize {
        let mut pos = Some(pos);

        for c in word {
            if let Some((row, col)) = pos {
                if self.at(row, col) != Some(*c) {
                    return 0;
                }

                pos = iter.next(row, col);
            } else {
                return 0;
            }
        }

        1
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];

        s.trim()
            .split_terminator("\n")
            .enumerate()
            .for_each(|(row, line)| {
                v.push(vec![]);
                line.trim()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(col, value)| {
                        v[row].push(Cell {
                            row,
                            col,
                            value: char::from(*value),
                        })
                    });
            });

        Ok(Grid {
            rows: v.len(),
            cols: v.first().unwrap().len(),
            grid: v,
        })
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = GridIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            offset: 0,
        }
    }
}

struct GridIntoIterator<'a> {
    grid: &'a Grid,
    offset: usize,
}

impl<'a> Iterator for GridIntoIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.offset / self.grid.cols;
        let col = self.offset % self.grid.cols;
        let cell = self.grid.cell(row, col);
        self.offset += 1;

        cell
    }
}

enum Searcher {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Searcher {
    pub fn next(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Searcher::Left => match col {
                0 => None,
                _ => Some((row, col - 1)),
            },
            Searcher::Right => Some((row, col + 1)),
            Searcher::UpRight => match row {
                0 => None,
                _ => Some((row - 1, col + 1)),
            },
            Searcher::UpLeft => match (row, col) {
                (0, _) => None,
                (_, 0) => None,
                _ => Some((row - 1, col - 1)),
            },
            Searcher::DownRight => Some((row + 1, col + 1)),
            Searcher::DownLeft => match col {
                0 => None,
                _ => Some((row + 1, col - 1)),
            },
            Searcher::Up => match row {
                0 => None,
                _ => Some((row - 1, col)),
            },
            Searcher::Down => Some((row + 1, col)),
        }
    }
}

#[cfg(test)]
mod validation {
    use crate::{solve, solve_complex, Grid, Searcher, DAY, WORKSPACE, XMAS};
    use std::str::FromStr;

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
    fn tst_grid() {
        let input = r#"
        X2345
        6M432
        12A45
        654S2"#;

        let g = Grid::from_str(input).unwrap();

        assert_eq!(g.cols, 5);
        assert_eq!(g.rows, 4);
        assert_eq!(g.grid[0][0].value, 'X');
        assert_eq!(g.grid[3][4].value, '2');
        assert_eq!(g.grid[3][4].row, 3);
        assert_eq!(g.grid[3][4].col, 4);
    }

    #[test]
    fn tst_grid_iter() {
        let input = r#"
        X2345
        6M432
        12A45
        654S2"#;

        let g = Grid::from_str(input).unwrap();
        let total = g.into_iter().count();

        assert_eq!(total, 20);
        assert_eq!(g.into_iter().nth(6).unwrap().value, 'M');
    }

    #[test]
    fn tst_grid_contains_down() {
        let grid = Grid::from_str(
            r#"
        X234X
        MM4M2
        A2A45
        SS4S2"#,
        )
        .unwrap();

        assert_eq!(grid.count((0, 0), &XMAS, Searcher::Down), 1);
        assert_eq!(grid.count((0, 4), &XMAS, Searcher::Down), 0);
        assert_eq!(grid.count((0, 4), &XMAS, Searcher::DownLeft), 1);
        assert_eq!(grid.count((0, 0), &XMAS, Searcher::DownLeft), 0);
        assert_eq!(grid.count((0, 0), &XMAS, Searcher::DownRight), 1);
        assert_eq!(grid.count((0, 4), &XMAS, Searcher::DownRight), 0);
    }

    #[test]
    fn tst_grid_contains_up() {
        let grid = Grid::from_str(
            r#"
        S234S
        AA4A2
        M2M45
        XX4X2"#,
        )
        .unwrap();

        assert_eq!(grid.count((3, 3), &XMAS, Searcher::UpLeft), 1);
        assert_eq!(grid.count((3, 0), &XMAS, Searcher::UpLeft), 0);
        assert_eq!(grid.count((3, 0), &XMAS, Searcher::Up), 1);
        assert_eq!(grid.count((3, 1), &XMAS, Searcher::Up), 0);
        assert_eq!(grid.count((3, 1), &XMAS, Searcher::UpRight), 1);
        assert_eq!(grid.count((0, 0), &XMAS, Searcher::UpRight), 0);
    }

    #[test]
    fn tst_grid_contains_leftright() {
        let grid = Grid::from_str(
            r#"
        00000
        XMAS0
        0SAMX
        00000"#,
        )
        .unwrap();

        assert_eq!(grid.count((2, 4), &XMAS, Searcher::Left), 1);
        assert_eq!(grid.count((1, 0), &XMAS, Searcher::Right), 1);
        assert_eq!(grid.count((1, 0), &XMAS, Searcher::Left), 0);
        assert_eq!(grid.count((2, 4), &XMAS, Searcher::Right), 0);
    }
}
