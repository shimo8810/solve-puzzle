use std::collections::HashSet;
use std::io;

const EMPTY: i8 = -1;
const SIZE: usize = 9;
type Field = [[i8; SIZE]; SIZE];

struct Sudoku(Field);

impl Sudoku {
    fn put(&mut self, x: usize, y: usize, val: i8) {
        self.0[y][x] = val;
    }

    fn reset(&mut self, x: usize, y: usize) {
        self.0[y][x] = EMPTY;
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == EMPTY {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn find_choices(&self, x: usize, y: usize) -> Vec<i8> {
        let mut set: HashSet<_> = (1i8..=9).collect();

        let u = x / 3 * 3;
        let v = y / 3 * 3;
        for i in 0..9 {
            set.remove(&self.0[y][i]);
            set.remove(&self.0[i][x]);
            set.remove(&self.0[v + i / 3][u + i % 3]);
        }

        set.into_iter().collect()
    }

    fn show(&self) -> String {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn dfs(sudoku: &mut Sudoku, ans: &mut Vec<String>) {
    if let Some((x, y)) = sudoku.find_empty() {
        for val in sudoku.find_choices(x, y).into_iter() {
            sudoku.put(x, y, val);
            dfs(sudoku, ans);
            sudoku.reset(x, y);
        }
    } else {
        ans.push(sudoku.show());
    }
}

pub fn solve(puzzle: Vec<String>) -> Vec<String> {
    let field = parse_puzzle(puzzle);

    let mut sudoku = Sudoku(field);
    let mut ans = vec![];
    dfs(&mut sudoku, &mut ans);

    ans
}

pub fn input_puzzle() -> Result<Vec<String>, &'static str> {
    //
    let mut puzzle = vec![];

    let stdin = io::stdin(); // We get `Stdin` here.
    for _ in 0..9 {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let row = buffer.trim().to_string();
        if row.len() != 9 || !row.chars().all(|c| matches!(c, '1'..='9' | '*')) {
            return Err("input error error");
        }
        puzzle.push(row);
    }

    Ok(puzzle)
}

fn parse_puzzle(puzzle: Vec<String>) -> Field {
    let mut field = [[-1; SIZE]; SIZE];
    for (y, row) in puzzle.iter().enumerate() {
        for (x, val) in row.chars().enumerate() {
            field[y][x] = match val {
                '1'..='9' => val as i8 - '0' as i8,
                _ => -1,
            };
        }
    }

    field
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILED: Field = [
        [5, 3, -1, -1, 7, -1, -1, -1, -1],
        [6, -1, -1, 1, 9, 5, -1, -1, -1],
        [-1, 9, 8, -1, -1, -1, -1, 6, -1],
        [8, -1, -1, -1, 6, -1, -1, -1, 3],
        [4, -1, -1, 8, -1, 3, -1, -1, 1],
        [7, -1, -1, -1, 2, -1, -1, -1, 6],
        [-1, 6, -1, -1, -1, -1, 2, 8, -1],
        [-1, -1, -1, 4, 1, 9, -1, -1, 5],
        [-1, -1, -1, -1, 8, -1, -1, 7, 9],
    ];

    #[test]
    fn test_put() {
        let mut sudoku = Sudoku(FILED);
        sudoku.put(0, 0, 1);
        assert_eq!(sudoku.0[0][0], 1);
    }

    #[test]
    fn test_find_choices() {
        let sudoku = Sudoku(FILED);
        let mut choices = sudoku.find_choices(2, 0);
        choices.sort_unstable();
        assert_eq!(choices, [1, 2, 4]);
    }
}
