use puzzle15::{input_puzzle, solve};

fn main() {
    let puzzle = input_puzzle().unwrap();
    if let Some(ans) = solve(puzzle) {
        for (i, p) in ans.iter().enumerate() {
            println!("-----{:02}-----", i);
            println!("{}", p);
        }
    } else {
        println!("not fond");
    }
}
