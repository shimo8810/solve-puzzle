#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Add,
    Sub,
    Mul,
    Div,
    Null,
}

use Operand::*;

fn check(ops: &[Operand], target: f64) -> bool {
    let eps = 1e-9;

    let (nums, ops) = calc_null(ops);
    let (nums, ops) = calc_mul_div(nums, ops);
    let ans = calc_add_sub(nums, ops);

    (ans - target).abs() <= eps
}

fn decode(ops: &[Operand]) -> String {
    let mut res = "1".to_string();
    for (i, &op) in ops.iter().enumerate() {
        match op {
            Add => res += " + ",
            Sub => res += " - ",
            Mul => res += " * ",
            Div => res += " / ",
            _ => {}
        }
        res += &(i + 2).to_string();

        //
    }

    res
}
fn rec(ops: &mut Vec<Operand>, ans: &mut Vec<String>) {
    if ops.len() == 8 {
        // 調べる
        if check(ops, 100.0) {
            ans.push(decode(ops));
        }
        return;
    }

    for op in [Add, Sub, Mul, Div, Null] {
        ops.push(op);
        rec(ops, ans);
        ops.pop().unwrap();
    }
}

pub fn solve() -> Vec<String> {
    let mut ops = vec![];
    let mut ans = vec![];
    rec(&mut ops, &mut ans);

    ans
}

fn calc_null(ops: &[Operand]) -> (Vec<f64>, Vec<Operand>) {
    let mut nums = vec![];

    let mut val = 1.0;
    for (i, &op) in ops.iter().enumerate() {
        let x = i as f64 + 2.0;
        match op {
            Null => val = val * 10. + x,
            _ => {
                //
                nums.push(val);
                val = x;
            }
        };
    }
    nums.push(val);
    let ops = ops.iter().cloned().filter(|&x| x != Null).collect();
    (nums, ops)
}

fn calc_mul_div(nums: Vec<f64>, ops: Vec<Operand>) -> (Vec<f64>, Vec<Operand>) {
    let mut new_nums = vec![];

    let mut val = nums[0];

    for (&n, &op) in nums.iter().skip(1).zip(ops.iter()) {
        match op {
            Mul => val *= n,
            Div => val /= n,
            _ => {
                new_nums.push(val);
                val = n;
            }
        };
    }
    new_nums.push(val);

    let ops = ops
        .iter()
        .cloned()
        .filter(|&x| x != Mul && x != Div)
        .collect();

    (new_nums, ops)
    //
}

fn calc_add_sub(nums: Vec<f64>, ops: Vec<Operand>) -> f64 {
    let mut val = nums[0];

    for (&n, &op) in nums.iter().skip(1).zip(ops.iter()) {
        match op {
            Add => val += n,
            Sub => val -= n,
            _ => {}
        };
    }

    val
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_null() {
        let ops = vec![Add, Null, Null, Mul, Div, Sub, Sub, Null];
        let (nums, ops) = calc_null(&ops);
        assert_eq!(nums, [1., 234., 5., 6., 7., 89.]);
        assert_eq!(ops, [Add, Mul, Div, Sub, Sub]);
    }

    #[test]
    fn test_calc_mul_div() {
        let nums = vec![1., 234., 5., 6., 7., 89.];
        let ops = vec![Add, Mul, Div, Sub, Sub];
        let (nums, ops) = calc_mul_div(nums, ops);
        assert_eq!(nums, [1., 195., 7., 89.]);
        assert_eq!(ops, [Add, Sub, Sub]);
    }

    #[test]
    fn test_calc_add_sub() {
        let nums = vec![1., 195., 7., 89.];
        let ops = vec![Add, Sub, Sub];
        let ans = calc_add_sub(nums, ops);
        assert_eq!(ans, 100.0);
    }
}
