use std::io;

type Pos = (usize, usize);

fn get_digit(pos: &Pos) -> u64 {
    ((pos.0 * 4 + pos.1) * 4) as u64
}

fn get_val(puzzle: u64, pos: &Pos) -> u8 {
    ((puzzle >> get_digit(pos)) & 0b1111) as u8
}

pub fn input_puzzle() -> Result<Vec<Vec<u8>>, &'static str> {
    //
    let mut puzzle = vec![];

    let stdin = io::stdin(); // We get `Stdin` here.
    for _ in 0..4 {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let row = buffer.trim().to_string();

        if let Ok(row) = row
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<Vec<u8>, _>>()
        {
            if row.len() != 4 {
                return Err("input error");
            }
            puzzle.push(row);
        } else {
            return Err("input error");
        }
    }

    Ok(puzzle)
}

pub fn solve(puzzle: Vec<Vec<u8>>) -> Option<Vec<String>> {
    let (puzzle, emp) = encode_puzzle(puzzle);
    let est = estimate(puzzle);
    for maxdepth in 1..=80 {
        let mut ans = vec![];

        iddfs(maxdepth, 0, puzzle, &emp, est, 5, &mut ans);

        if !ans.is_empty() {
            return Some(ans.into_iter().rev().collect());
        }
    }

    None
}

fn encode_puzzle(puzzle: Vec<Vec<u8>>) -> (u64, Pos) {
    let mut code = 0u64;
    let mut pos = (0, 0);
    for (x, row) in puzzle.into_iter().enumerate() {
        for (y, val) in row.into_iter().enumerate() {
            if val == 0 {
                pos = (x, y);
            }
            code += (val as u64) << get_digit(&(x, y));
        }
    }

    (code, pos)
}

fn show(puzzle: u64) -> String {
    (0..4)
        .map(|x| {
            (0..4)
                .map(|y| format!("{:2}", get_val(puzzle, &(x, y))))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn slide(mut puzzle: u64, val: u8, pos: &Pos, emp: &Pos) -> u64 {
    let val = val as u64;
    puzzle -= val << get_digit(pos);
    puzzle += val << get_digit(emp);
    puzzle
}

fn calc_distance(val: u8, pos: &Pos) -> usize {
    let x = ((val - 1) / 4) as i8;
    let y = ((val - 1) % 4) as i8;

    ((x - pos.0 as i8).abs() + (y - pos.1 as i8).abs()) as usize
}

fn estimate(puzzle: u64) -> usize {
    let mut res = 0;

    for x in 0..4 {
        for y in 0..4 {
            let val = get_val(puzzle, &(x, y));
            if val != 0 {
                res += calc_distance(val, &(x, y));
            }
        }
    }

    res as usize
}

fn estimate_next(est: usize, val: u8, pos: &Pos, emp: &Pos) -> usize {
    (est + calc_distance(val, emp) - calc_distance(val, pos)) as usize
}

const DXY: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn iddfs(
    maxdepth: usize,
    depth: usize,
    puzzle: u64,
    emp: &Pos,
    est: usize,
    predir: usize,
    ans: &mut Vec<String>,
) {
    //
    if !ans.is_empty() {
        return;
    }

    if est == 0 {
        ans.push(show(puzzle));
        return;
    }

    if depth >= maxdepth {
        return;
    }

    for dir in 0..4 {
        let revdir = (dir + 2) % 4;
        if revdir == predir {
            continue;
        }
        let nx = emp.0 as isize + DXY[dir].0;
        let ny = emp.1 as isize + DXY[dir].1;

        if !(0..4).contains(&nx) || !(0..4).contains(&ny) {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        let pos = &(nx, ny);
        let val = get_val(puzzle, pos);
        let new_puzzle = slide(puzzle, val, pos, emp);
        let next_est = estimate_next(est, val, pos, emp);
        if depth + next_est <= maxdepth {
            iddfs(maxdepth, depth + 1, new_puzzle, pos, next_est, dir, ans);

            if !ans.is_empty() {
                ans.push(show(puzzle));
                return;
            }
        }
    }
}
