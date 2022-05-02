use make10::solve;
use std::env;

fn main() {
    let numbers: Result<Vec<u32>, _> = env::args().skip(1).map(|x| x.parse()).collect();

    if let Ok(numbers) = numbers {
        if numbers.len() != 4 {
            eprintln!("illegal length of numbers.");
            return;
        }
        let ans = solve(&numbers, 10);
        for exp in &ans {
            println!("{}", exp);
        }
    } else {
        eprintln!("illegal arguments.");
    }
}
