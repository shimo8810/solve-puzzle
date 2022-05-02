use itertools::Itertools;
use std::collections::HashSet;

fn calc_rpn(exp: &str) -> Result<f64, &str> {
    let errmsg = "calculate error";

    let mut stack = vec![];

    for c in exp.chars() {
        let v = match c {
            '0'..='9' => c.to_digit(10).unwrap() as f64,
            _ => {
                let y = match stack.pop() {
                    Some(v) => v,
                    None => return Err(errmsg),
                };
                let x = match stack.pop() {
                    Some(v) => v,
                    None => return Err(errmsg),
                };
                match c {
                    '+' => x + y,
                    '-' => x - y,
                    '*' => x * y,
                    '/' => x / y,
                    _ => return Err(errmsg),
                }
            }
        };
        stack.push(v);
    }
    stack.pop().ok_or(errmsg)
}

fn decode_rpn(exp: &str) -> Result<String, &str> {
    let errmsg = "decode error";
    let mut stack: Vec<String> = vec![];

    for c in exp.chars() {
        let s = match c {
            '0'..='9' => c.to_string(),
            _ => {
                let mut y = match stack.pop() {
                    Some(v) => v,
                    None => return Err(errmsg),
                };
                let mut x = match stack.pop() {
                    Some(v) => v,
                    None => return Err(errmsg),
                };
                match c {
                    '*' | '/' => {
                        if x.len() > 1 {
                            x = format!("({x})");
                        }
                        if y.len() > 1 {
                            y = format!("({y})");
                        }
                        format!("{x}{c}{y}")
                    }
                    '+' | '-' => format!("{x}{c}{y}"),
                    _ => return Err(errmsg),
                }
            }
        };
        stack.push(s);
    }
    stack.pop().ok_or(errmsg)
}

fn check(exp: &str, target: u32) -> bool {
    let eps = 1e-4;

    (calc_rpn(exp).unwrap() - target as f64).abs() <= eps
}

pub fn solve(numbers: &[u32], target: u32) -> Vec<String> {
    let mut ans = HashSet::new();

    let ops = ['+', '-', '*', '/'];
    for n in numbers.iter().permutations(numbers.len()) {
        for &o0 in &ops {
            for &o1 in &ops {
                for &o2 in &ops {
                    let exp = format!("{}{}{}{}{}{}{}", n[0], n[1], n[2], n[3], o0, o1, o2);
                    if check(&exp, target) {
                        ans.insert(decode_rpn(&exp).unwrap());
                    }

                    let exp = format!("{}{}{}{}{}{}{}", n[0], n[1], n[2], o0, n[3], o1, o2);
                    if check(&exp, target) {
                        ans.insert(decode_rpn(&exp).unwrap());
                    }

                    let exp = format!("{}{}{}{}{}{}{}", n[0], n[1], n[2], o0, o1, n[3], o2);

                    if check(&exp, target) {
                        ans.insert(decode_rpn(&exp).unwrap());
                    }

                    let exp = format!("{}{}{}{}{}{}{}", n[0], n[1], o0, n[2], o1, n[3], o2);
                    if check(&exp, target) {
                        ans.insert(decode_rpn(&exp).unwrap());
                    }

                    let exp = format!("{}{}{}{}{}{}{}", n[0], n[1], o0, n[2], n[3], o1, o2);
                    if check(&exp, target) {
                        ans.insert(decode_rpn(&exp).unwrap());
                    }
                }
            }
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_rpn() {
        assert_eq!(calc_rpn("62+"), Ok(8.0));
        assert_eq!(calc_rpn("62-"), Ok(4.0));
        assert_eq!(calc_rpn("62*"), Ok(12.0));
        assert_eq!(calc_rpn("62/"), Ok(3.0));
        assert_eq!(calc_rpn("8115/-/"), Ok(8. / (1. - 1. / 5.)));
        assert_eq!(calc_rpn("374/-8*"), Ok((3. - 7. / 4.) * 8.));
        assert_eq!(calc_rpn("57*55*-"), Ok(5. * 7. - 5. * 5.));

        assert_eq!(calc_rpn("*/"), Err("calculate error"));
        assert_eq!(calc_rpn("abcd"), Err("calculate error"));
    }

    #[test]
    fn test_decode_rpn() {
        assert_eq!(decode_rpn("62+"), Ok("6+2".to_string()));
        assert_eq!(decode_rpn("62-"), Ok("6-2".to_string()));
        assert_eq!(decode_rpn("62*"), Ok("6*2".to_string()));
        assert_eq!(decode_rpn("62/"), Ok("6/2".to_string()));
        assert_eq!(decode_rpn("8115/-/"), Ok("8/(1-1/5)".to_string()));
        assert_eq!(decode_rpn("374/-8*"), Ok("(3-7/4)*8".to_string()));
        assert_eq!(decode_rpn("57*55*-"), Ok("5*7-5*5".to_string()));

        assert_eq!(decode_rpn("*/"), Err("decode error"));
        assert_eq!(decode_rpn("abcd"), Err("decode error"));
    }
}
