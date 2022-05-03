use sudoku::{input_puzzle, solve};

fn main() {
    if let Ok(puzzle) = input_puzzle() {
        let ans = solve(puzzle);
        for s in &ans {
            println!("-----------------");
            println!("{}", s);
        }
    } else {
        eprintln!("illegal input");
    }
}
